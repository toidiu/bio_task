#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    itemId: i64,
    title: String,
    description: String,
}

impl Item {
    pub fn new(itemId: i64, title: String, description: String) -> Self {
        Item {
            itemId: itemId,
            title: title,
            description: description,
        }
    }
}
