mod cli;
mod config;
mod parser;
mod schedule;
mod utils;

use std::{
    fs::{self, File, OpenOptions},
    path::Path,
};

use anyhow::Result;
use calamine::{open_workbook, DataType, Error, Range, RangeDeserializerBuilder, Reader, Xlsx};
use clap::Parser;
use cli::Cli;
use config::{ConfigParams, CourseChecker};
use schedule::{ClassInfo, Schedule};

fn main() -> Result<()> {
    let cli = Cli::parse();

    dbg!(&cli);

    let Some(command) = cli.command else {
        return Ok(());
    };

    match command {
        cli::Commands::Parse {
            config,
            file,
            output,
        } => {
            let output_file =
                output.unwrap_or(format!("{}/result.json", env!("CARGO_MANIFEST_DIR")));


            parse_sheet(&file, &config, &output_file)?;

            println!("Successfuly parsed to {}", output_file);
        }
    }

    Ok(())
}

fn parse_sheet(input_file: &str, config_file: &str, result_file: &str) -> Result<()> {

    let skip = (1, 0);

    let mut workbook: Xlsx<_> = open_workbook(input_file)?;

    let first_sheet = workbook
        .sheet_names()
        .into_iter()
        .inspect(|v| println!("{}", v))
        .next()
        .unwrap();

    let sheet = workbook
        .worksheet_range(&first_sheet)
        .ok_or(Error::Msg("Couldn't get Sheet1"))??;

    sheet.range(skip, sheet.end().unwrap());


    let mut schedule = Schedule::new();
    search_for_table(sheet, &load_config(config_file)?, &mut schedule)?;

    save_data(result_file, &schedule)?;
    Ok(())
}

fn load_config(config_file: &str) -> Result<ConfigParams> {
    return Ok(serde_json::from_reader::<_, ConfigParams>(
        File::open(config_file).unwrap(),
    )?);
}

fn save_data(result_file: &str, schedule: &Schedule) -> Result<()> {
    if Path::new(result_file).exists() {
        fs::remove_file(result_file)?;
    }

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(result_file)?;

    serde_json::to_writer(file, schedule)?;

    Ok(())
}

fn search_for_table(
    table_search: Range<DataType>,
    course_checker: &impl CourseChecker,
    schedule: &mut Schedule,
) -> Result<()> {
    let mut iter = RangeDeserializerBuilder::new().from_range(&table_search)?;
    //schedule.specialties.insert("hello".into(), SepecialtySchedule{});

    let mut current_day = "".to_string();
    let mut current_hour = "".to_string();
    while let Some(result) = iter.next() {
        let (day, hour, class_title, groups, weeks, auditorium): (
            String,
            String,
            String,
            String,
            String,
            String,
        ) = result?;

        if !day.is_empty() {
            current_day = day.clone();
        }
        if !hour.is_empty() {
            current_hour = hour.clone();
        }

        dbg!(&class_title);

        if class_title.is_empty() {
            continue;
        }

        let specialties = course_checker.identify_sepecialties(&class_title)?;
        assert!(
            specialties.len() > 0,
            "Specialties shouldn't be less than 0"
        );
        let groups = course_checker.parse_groups(&specialties, &groups)?;

        let class_info = ClassInfo::new(
            class_title.clone(),
            current_hour.clone(),
            weeks,
            auditorium,
            current_day.clone(),
        );

        let normalized_class_title = course_checker.normalize_title(&class_title)?;

        for (k, v) in groups.iter() {
            schedule.put_group_for_specialty(k, &normalized_class_title, v, class_info.clone());
        }
    }

    Ok(())
}
