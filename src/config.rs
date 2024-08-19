use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub powersave: u32,
    pub balance: u32,
    pub performance: u32,
    pub fast: u32,
}
