
#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: PartialEq + Ord> CustomSet<T> {
    // Mutating operations:

    pub fn new(mut inp: Vec<T>) -> Self {
        inp.sort();
        inp.dedup();
        CustomSet { data: inp }
    }

    pub fn add(&mut self, elem: T) {
        if !self.contains(&elem) {
            self.data.push(elem);
            self.data.sort();
        }
    }


    // Checks:

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        if self.data == vec![] || other.data == vec![] {
            return true;
        }

        self.data.iter().all(|e| !other.contains(e))
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.data == vec![] {
            return true;
        }
        if other.data == vec![] {
            return false;
        }
        self.data.iter().all(|e| other.contains(e))
    }

    pub fn contains(&self, elem: &T) -> bool {
        (&self.data).contains(elem)
    }


    // Set operations:

    pub fn intersection(&self, other: &Self) -> Self
        where T: Clone
    {
        if self.data == vec![] || self.data == vec![] {
            return CustomSet::new(vec![]);
        }
        let temp: Vec<T> = self.data
            .iter()
            .filter(|e| other.contains(e))
            .cloned()
            .collect();
        CustomSet::new(temp)
    }

    pub fn difference(&self, other: &Self) -> Self
        where T: Clone
    {
        if (self.data == vec![] && other.data == vec![]) || self.data == vec![] {
            return CustomSet::new(vec![]);
        }
        let temp: Vec<T> = self.data
            .iter()
            .filter(|e| !other.contains(e))
            .cloned()
            .collect();
        CustomSet::new(temp)
    }

    pub fn union(&self, other: &Self) -> Self
        where T: Clone
    {
        if self.data == vec![] && other.data == vec![] {
            return CustomSet::new(vec![]);
        }
        let mut temp1 = self.data.clone();
        temp1.extend_from_slice(&other.data);

        CustomSet::new(temp1)
    }
}
