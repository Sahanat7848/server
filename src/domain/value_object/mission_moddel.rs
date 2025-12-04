use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::domain::{entities::missions::{AddMissionEntity, EditMissionEntity}, value_object::mission_statuses::MissionStatuses};
// ทำต่อนี่
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct MissionModel {
pub id: i32,
pub name: String,
pub description: Option<String>,
pub status: String,
pub chief_id: i32,
pub crew_count: i64,
pub created_at: NaiveDateTime,
pub updated_at: NaiveDateTime,
}

pub struct AddMissionModel {
    pub name:String,
    pub description: Option<String>,
}

impl AddMissionModel {
    pub fn to_entity(&self,chief_id:i32) -> AddMissionEntity {
        AddMissionEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            status: MissionStatuses::Open.to_string(),
            chief_id,
        }
    }
}


pub struct EditMissionModel {
    pub name:String,
    pub description: Option<String>,
    pub status: Option<String>,
}

impl EditMissionModel { // here have something slide 3-16
    pub fn to_entity(&self,chief_id:i32) -> EditMissionEntity {
        EditMissionEntity {
            name: Some(self.name.clone()),
            description: self.description.clone(),
            status: self.status.clone(),
            chief_id,
        }
    }
}
