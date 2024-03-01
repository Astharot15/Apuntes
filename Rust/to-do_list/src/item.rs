// Se define la estructura `Item`
#[derive(Debug, Clone)]
pub struct Item {
    pub title: String,
    pub description: String,
}

impl Item {
    pub fn new(title: String, description: String) -> Self {
        Self {
            title,
            description,
        }
    }
}