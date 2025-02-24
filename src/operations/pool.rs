use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, Serialize, Clone)]
/// The different tier levels of HashGate user pools.
pub enum PoolTier {
    /// A demo tier user pool.
    Demo,

    /// A hobby tier user pool.
    Hobby,

    /// A pro tier user pool.
    Pro,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
/// A HashGate user pool.
pub struct Pool {
    /// The uuid of the user pool.
    pub pool_id: Uuid,

    /// The name of the user pool.
    pub pool_name: String,

    /// The tier of the user pool.
    pub pool_tier: PoolTier,

    /// The creation date of the user pool.
    pub creation_date: NaiveDateTime,
}
