use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Job {
    pub id: String,
    pub input: String,
    pub result: String,
    pub status: JobStatus,
}

#[derive(Deserialize, Serialize)]
pub enum JobStatus {
    Pending,
    Created,
    Running,
    Succeeded,
    Failed,
}

impl From<u32> for JobStatus {
    fn from(value: u32) -> Self {
        match value {
            0 => JobStatus::Pending,
            1 => JobStatus::Running,
            2 => JobStatus::Succeeded,
            _ => JobStatus::Failed,
        }
    }
}

impl Into<i64> for JobStatus {
    fn into(self) -> i64 {
        self as i64
    }
}
