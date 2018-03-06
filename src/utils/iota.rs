#[derive(Clone)]
pub struct Path {
    is_absolute: bool,
    id_indexes: Vec<usize>,
    string: Option<String>
}

impl Path {
    pub fn new(is_absolute: bool, id_indexes: Vec<usize>) -> Path {
        Path { is_absolute, id_indexes, string: None }
    }

    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }
}
