use std::collections::BTreeMap;

use anyhow::Result;
use calamine::Error;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take, take_while1, take_while},
    error::ParseError,
    IResult,
};
use serde::{Deserialize, Serialize};

use crate::{parser::parse_title_specialties, utils::str_error_to_string};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecialtyConfig {
    aliases: Vec<String>,
    name: String,
}

impl SpecialtyConfig {
    fn matches(&self, potential_alias: &str) -> bool {
        let clean_pot_al = if potential_alias.ends_with('.') {
            &potential_alias[..potential_alias.len() - 1]
        } else {
            potential_alias
        };

        dbg!(&clean_pot_al);

        // Maybe optimize at some point
        let clean_name = self.name.to_lowercase();
        let clean_pot_al = clean_pot_al.to_lowercase();

        dbg!(
            &clean_name,
            &clean_pot_al,
            clean_name.starts_with(&clean_pot_al)
        );

        if clean_name.starts_with(&clean_pot_al) {
            return true;
        }

        for alias in &self.aliases {
            if alias == &clean_pot_al {
                return true;
            }
        }

        return false;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigParams {
    default_specialty: String,
    specialties_configs: Vec<SpecialtyConfig>,
}

impl ConfigParams {
    fn name_for_alias(&self, alias: &str) -> Option<String> {
        for course_alias_config in self.specialties_configs.iter() {
            dbg!(&course_alias_config, alias);
            if course_alias_config.matches(alias) {
                return Some(course_alias_config.name.clone());
            }
        }
        return None;
    }

    fn identify_specialties_impl<'a>(&self, title: &'a str) -> IResult<&'a str, Vec<String>> {
        let mut result_courses: Vec<String> = vec![];
        let mut value = title;
        dbg!(&value);
        while !value.is_empty() {
            dbg!(&value);
            let (input, _) = is_not("(")(value)?;

            if input.is_empty() {
                break;
            }

            dbg!(&input);
            let (input, _) = nom::bytes::complete::take(1usize)(input)?;

            dbg!(&input);
            let (rest, internal) = is_not(")")(input)?;

            let (_, items) = parse_title_specialties(internal)?;

            dbg!(&items);
            for item in items {
                if let Some(name) = self.name_for_alias(&item) {
                    result_courses.push(name);
                }
            }

            value = rest
        }
        if result_courses.is_empty() {
            result_courses.push(self.default_specialty.clone());
        }

        dbg!(&result_courses);
        dbg!(&self.specialties_configs);
        assert!(result_courses.iter().all(|v| self
            .specialties_configs
            .iter()
            .map(|specialty| &specialty.name)
            .any(|specialty_name| specialty_name == v)));

        Ok((value, result_courses))
    }

    fn normalize_title_impl<'a>(&self, title: &'a str) -> IResult<&'a str, String> {
        let mut text = String::new();

        let mut rest_of_title = title;

        while !rest_of_title.is_empty() {
            let (input, next_chunk) = take_while(|v: char| v != '(')(rest_of_title)?;

            text.push_str(next_chunk);

            if input.is_empty() {
                break;
            }

            let (input, _) = take(1usize)(input)?;
            let (input, internal) = take_while1(|v: char| v != ')')(input)?;
            let (input, _) = take(1usize)(input)?;

            if !self.has_course_names_inside(internal) {
                text.push('(');
                text.push_str(internal);
                text.push(')');

                rest_of_title = input;
            } else {

                let space: IResult<&str, &str> = tag(" ")(input);
                if let Ok((skipped_space_input, _)) = space {
                    rest_of_title = skipped_space_input
                }
                else {
                    rest_of_title = input
                }
            }
        }
        return Ok(("", text));
    }

    fn has_course_names_inside(&self, text: &str) -> bool {
        let (_, items) = match parse_title_specialties(text) {
            Ok(v) => v,
            Err(_) => return false,
        };

        for item in items {
            if let Some(name) = self.name_for_alias(&item) {
                return true;
            }
        }
        return false;
    }
}

fn parse_group_number(input: &str) -> IResult<&str, &str> {
    //nom::bytes::complete::tag_no_case
    dbg!(input);

    let (input, group) = alt((
        tag_no_case("лекція"),
        take_while1(|c: char| c.is_digit(10) || c == '-' || c == ','),
    ))(input)?;
    //let (input, number) = take_till(|c: char| c.is_digit(10) || c == '-')(input)?;

    Ok((input, group))
}

impl CourseChecker for ConfigParams {
    fn identify_sepecialties(&self, title: &str) -> Result<Vec<String>> {
        let (_, result) = str_error_to_string(self.identify_specialties_impl(title))?;
        return Ok(result);
    }

    fn parse_groups(
        &self,
        specialties: &Vec<String>,
        group: &str,
    ) -> Result<BTreeMap<String, String>> {
        // In the future add parsing for cases like "1 ф., 2 е."
        let group = group.trim();
        let group_num = {
            let (_, parse_result) = str_error_to_string(parse_group_number(group))?;
            parse_result
        };

        let mut map = BTreeMap::new();

        for v in specialties {
            map.insert(v.clone(), group_num.to_string());
        }

        Ok(map)
    }

    fn normalize_title(&self, title: &str) -> Result<String> {
        let (_, result) = str_error_to_string(self.normalize_title_impl(title))?;
        Ok(result)
    }
}

pub trait CourseChecker {
    fn identify_sepecialties(&self, title: &str) -> Result<Vec<String>>;

    fn parse_groups(
        &self,
        specialties: &Vec<String>,
        group: &str,
    ) -> Result<BTreeMap<String, String>>;

    fn normalize_title(&self, title: &str) -> Result<String>;
}

#[cfg(test)]
mod tests {
    use crate::config::SpecialtyConfig;

    use super::{ConfigParams, CourseChecker};

    #[test]
    fn test_default_identify_specialites() {
        let config = ConfigParams {
            default_specialty: "Default".to_string(),
            specialties_configs: vec![SpecialtyConfig {
                name: "Default".to_string(),
                aliases: vec![],
            }],
        };

        let result = config.identify_sepecialties("abc (def)");
        assert_eq!(result.unwrap(), vec!["Default".to_string()])
    }

    #[test]
    fn test_identify_specialties() {
        {
            let config = ConfigParams {
                default_specialty: "Default".to_string(),
                specialties_configs: vec![
                    SpecialtyConfig {
                        aliases: vec![],
                        name: "Default".to_string(),
                    },
                    SpecialtyConfig {
                        aliases: vec![],
                        name: "Hello".to_string(),
                    },
                    SpecialtyConfig {
                        aliases: vec![],
                        name: "There".to_string(),
                    },
                ],
            };

            let result = config.identify_sepecialties("abc (Hello., There.)");
            assert_eq!(
                result.unwrap(),
                vec!["Hello".to_string(), "There".to_string()]
            );
        }

        {
            let config = ConfigParams {
                default_specialty: "Default".to_string(),
                specialties_configs: vec![
                    SpecialtyConfig {
                        aliases: vec![],
                        name: "Default".to_string(),
                    },
                    SpecialtyConfig {
                        name: "Маркетинг".to_string(),
                        aliases: vec![],
                    },
                ],
            };

            let result = config.identify_sepecialties(
                "Digital – маркетинг (марк.) доц. Пічик К.В., доц. Козченко Я.В., ст.викл. Мельник В.В.",
            );
            assert_eq!(result.unwrap(), vec!["Маркетинг".to_string()]);
        }

        {
            let config = ConfigParams {
                default_specialty: "Default".to_string(),
                specialties_configs: vec![SpecialtyConfig {
                    aliases: vec![],
                    name: "Default".to_string(),
                }],
            };

            let result = config.identify_sepecialties(
                "Методи об'єктно-орієнтованого програмування, доц. В.В. Бублик",
            );
            assert_eq!(result.unwrap(), vec!["Default".to_string()]);
        }

        {
            let config = ConfigParams {
                default_specialty: "Error".to_string(),
                specialties_configs: vec![
                    SpecialtyConfig {
                        name: "Економіка".to_string(),
                        aliases: vec![],
                    },
                    SpecialtyConfig {
                        name: "Маркетинг".to_string(),
                        aliases: vec![],
                    },
                    SpecialtyConfig {
                        name: "Менеджмент".to_string(),
                        aliases: vec![],
                    },
                    SpecialtyConfig {
                        name: "Фінанси".to_string(),
                        aliases: vec![],
                    },
                ],
            };

            let result = config.identify_sepecialties(
                "Digital – маркетинг (марк.) доц. Пічик К.В., доц. Козченко Я.В., ст.викл. Мельник В.В.",
            );
            assert_eq!(result.unwrap(), vec!["Маркетинг".to_string()]);
        }
    }

    #[test]
    fn test_alias() {
        let alias = SpecialtyConfig {
            aliases: vec![],
            name: "Маркетинг".to_string(),
        };

        assert!(alias.matches("Марк."));
        assert!(alias.matches("марк."));
        assert!(alias.matches("Мар."));
        assert!(alias.matches("М"));
        assert!(alias.matches("Ма"));
    }
}
