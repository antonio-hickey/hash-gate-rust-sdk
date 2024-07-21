use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum PoolTier {
    Demo,
    Hobby,
    Pro,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    pub pool_id: Uuid,
    pub pool_name: String,
    pub pool_tier: PoolTier,
    pub creation_date: NaiveDateTime,
}
