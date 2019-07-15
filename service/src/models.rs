use chrono::prelude::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    itemId: i64,
    title: String,
    description: String,
    statusKey: i32,
    deadlineDate: NaiveDate,
    memberName: String,
    // `itemParentId` int(10) unsigned NOT NULL DEFAULT '0',
    // `priority` tinyint(3) unsigned NOT NULL DEFAULT '0',
    // `context` varchar(80) NOT NULL DEFAULT '',
    // `expectedDuration` smallint(5) unsigned NOT NULL DEFAULT '0',
    // `showInCalendar` tinyint(1) unsigned NOT NULL DEFAULT '0',
    // `showPrivate` tinyint(1) unsigned NOT NULL DEFAULT '0',
    // `authorId` mediumint(8) unsigned NOT NULL DEFAULT '0',
}

impl Item {
    pub fn new(
        itemId: i64,
        title: String,
        description: String,
        statusKey: i32,
        deadlineDate: NaiveDate,
        memberName: String,
    ) -> Self {
        Item {
            itemId,
            title,
            description,
            statusKey,
            deadlineDate,
            memberName,
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
