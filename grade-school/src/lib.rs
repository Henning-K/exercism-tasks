use std::collections::BTreeMap;

pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> Self {
        School { grades: BTreeMap::new() }
    }

    pub fn grade(&self, n: u32) -> Option<&Vec<String>> {
        self.grades.get(&n)
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut temp: Vec<u32> = self.grades.keys().cloned().collect();
        temp.sort();
        temp
    }

    pub fn add<S: Into<String>>(&mut self, grade: u32, name: S) {
        let name = name.into();
        if let Some(names) = self.grades.get_mut(&grade) {
            (*names).push(name);
            (*names).sort();
            return;
        }
        self.grades.insert(grade, vec![name]);
    }
}
