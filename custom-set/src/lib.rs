pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T> CustomSet<T> {
    pub fn new(inp: Vec<T>) -> Self {
        CustomSet { data: inp }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn contains(&self, elem: &T) -> bool {
        (&self.data).contains(elem)
    }
}
