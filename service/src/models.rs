#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    itemId: i64,
    title: String,
    description: String,
}

impl Item {
    pub fn new(itemId: i64, title: String, description: String) -> Self {
        Item {
            itemId,
            title,
            description,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    projectId: i64,
    name: String,
    description: String,
}

impl Project {
    pub fn new(projectId: i64, name: String, description: String) -> Self {
        Project {
            projectId,
            name,
            description,
        }
    }
}
