use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::db::Database;
use crate::freedesktop::desktop::DesktopEntry;

pub trait Entity {
    fn get_key(&self) -> Vec<u8>;
    fn get_weight(&self) -> u8;
    const COLLECTION: &'static str;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DesktopEntryEntity {
    pub name: String,
    pub icon: String,
    pub path: PathBuf,
    pub weight: u8,
}

impl DesktopEntryEntity {
    pub fn persist(entry: &DesktopEntry, path: &Path, db: &Database) {
        let weight = match db.get_by_key::<DesktopEntryEntity>(&entry.name) {
            Some(de_entry) => de_entry.weight + 1,
            None => 0,
        };

        let entity = Self {
            name: entry.name.clone(),
            icon: entry.icon.clone(),
            path: path.into(),
            weight,
        };

        debug!(
            "Inserting {:?} in {}",
            entity,
            DesktopEntryEntity::COLLECTION
        );
        db.insert(&entity).expect("Unable to insert history entry");
    }
}

impl Entity for DesktopEntryEntity {
    fn get_key(&self) -> Vec<u8> {
        self.name.as_bytes().to_vec()
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }

    const COLLECTION: &'static str = "desktop_entry";
}