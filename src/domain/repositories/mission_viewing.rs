use crate::domain::entities::missions::MissionEntity;
use crate::domain::value_object::mission_filter::MissionFilter;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait MissionViewingRepository {
    async fn get_one(&self, mission_id: i32) -> Result<MissionEntity>;
    async fn get_all(&self, mission_filter: &MissionFilter) -> Result<Vec<MissionEntity>>;
    async fn crew_counting(&self, mission_id: i32) -> Result<u32>;
}
