use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(flatten)]
    specialties: BTreeMap<String, SepecialtySchedule>,
}

impl Schedule {
    pub fn new() -> Self { Self { specialties: BTreeMap::new() } }

    pub fn put_group_for_specialty(
        &mut self,
        specialty_name: &str,
        class_name: &str,
        group: &str,
        class_info: ClassInfo,
    ) {
        if !self.specialties.contains_key(specialty_name) {
            self.specialties.insert(specialty_name.to_string(), SepecialtySchedule::new());
        }

        let specialty = self.specialties.get_mut(specialty_name).unwrap();

        //dbg!(&specialty);

        if !specialty.classes.contains_key(class_name) {
            specialty.classes.insert(class_name.to_string(), ClassSchedule::new());
        }
        
        let class = specialty.classes.get_mut(class_name).unwrap();

        //dbg!(&class);

        if !class.group_schedule.contains_key(group) {
            class.group_schedule.insert(group.to_string(), Vec::new());
        }
        
        class.group_schedule.get_mut(group).unwrap().push(class_info);

    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SepecialtySchedule {
    #[serde(flatten)]
    classes: BTreeMap<String, ClassSchedule>,
}

impl SepecialtySchedule {
    fn new() -> Self { Self { classes: BTreeMap::new() } }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassSchedule {
    #[serde(flatten)]
    group_schedule: BTreeMap<String, Vec<ClassInfo>>,
}

impl ClassSchedule {
    fn new() -> Self { Self { group_schedule: BTreeMap::new() } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassInfo {
    name: String,
    time: String,
    weeks: String,
    auditorium: String,
    day_of_week: String
}

impl ClassInfo {
    pub fn new(name: String, time: String, weeks: String, auditorium: String, day_of_week: String) -> Self { Self { name, time, weeks, auditorium, day_of_week } }
}
