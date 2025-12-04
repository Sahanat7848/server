use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    constants::MAX_CREW_PER_MISSION,
    entities::crew_memberships::CrewMemberShips,
    repositories::{
        crew_oparation::CrewOperationRepository, mission_viewing::MissionViewingRepository,
        transaction_provider::TransactionProvider,
    },
    value_object::mission_statuses::MissionStatuses,
};

pub struct CrewOperationUseCase<T1, T2>
where
    T1: CrewOperationRepository + TransactionProvider + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    crew_operation_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
}
impl<T1, T2> CrewOperationUseCase<T1, T2>
where
    T1: CrewOperationRepository + TransactionProvider + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    pub fn new(crew_operation_repository: Arc<T1>, mission_viewing_repository: Arc<T2>) -> Self {
        Self {
            crew_operation_repository,
            mission_viewing_repository,
        }
    }

    pub async fn join(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let mission = self.mission_viewing_repository.get_one(mission_id).await?;

        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;

        let mission_status_condition = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();
        if !mission_status_condition {
            return Err(anyhow::anyhow!("Mission is not joinable"));
        }
        let crew_count_condition = crew_count < MAX_CREW_PER_MISSION;
        if !crew_count_condition {
            return Err(anyhow::anyhow!("Mission is full"));
        }

        self.crew_operation_repository
            .join(CrewMemberShips {
                mission_id,
                brawler_id,
            })
            .await?;

        Ok(())
    }

    pub async fn leave(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let mission = self.mission_viewing_repository.get_one(mission_id).await?;

        let leaving_condition = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();
        if !leaving_condition {
            return Err(anyhow::anyhow!("Mission is not leavable"));
        }
        self.crew_operation_repository
            .leave(CrewMemberShips {
                mission_id,
                brawler_id,
            })
            .await?;

        Ok(())
    }
    pub async fn insert_and_delete_transaction(
        &self,
        mission_id: i32,
        brawler_id: i32,
    ) -> Result<()> {
        let repo = Arc::clone(&self.crew_operation_repository);
        let repo_for_closure = Arc::clone(&repo);

        repo.transaction::<(), anyhow::Error>(Box::new(move |conn| {
            repo_for_closure.for_insert_transaction_test(
                conn,
                CrewMemberShips {
                    mission_id,
                    brawler_id,
                },
            )?;

            repo_for_closure.for_delete_transaction_test(
                conn,
                CrewMemberShips {
                    mission_id,
                    brawler_id,
                },
            )?;

            Ok::<(), anyhow::Error>(())
        }))
        .await
    }
}
