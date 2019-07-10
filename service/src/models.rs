#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    itemId: i64,
    title: String,
    description: String,
  // `itemId` int(10) unsigned NOT NULL AUTO_INCREMENT,
  // `projectId` mediumint(8) unsigned NOT NULL DEFAULT '0',
  // `itemParentId` int(10) unsigned NOT NULL DEFAULT '0',
  // `priority` tinyint(3) unsigned NOT NULL DEFAULT '0',
  // `context` varchar(80) NOT NULL DEFAULT '',
  // `title` varchar(255) NOT NULL DEFAULT '',
  // `description` text NOT NULL,
  // `deadlineDate` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  // `expectedDuration` smallint(5) unsigned NOT NULL DEFAULT '0',
  // `showInCalendar` tinyint(1) unsigned NOT NULL DEFAULT '0',
  // `showPrivate` tinyint(1) unsigned NOT NULL DEFAULT '0',
  // `memberId` mediumint(8) unsigned NOT NULL DEFAULT '0',
  // `authorId` mediumint(8) unsigned NOT NULL DEFAULT '0',
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
