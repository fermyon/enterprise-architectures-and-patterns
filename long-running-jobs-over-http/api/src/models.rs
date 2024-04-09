use serde::{Deserialize, Serialize};
use shared::{Job, JobStatus};
use spin_sdk::{http::conversions::IntoBody, sqlite::Row};

impl From<CreateJobModel> for Job {
    fn from(value: CreateJobModel) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            input: value.input,
            result: String::new(),
            status: JobStatus::Pending,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct CreateJobModel {
    pub input: String,
}

#[derive(Serialize)]
pub(crate) struct JobStatusModel {
    pub id: String,
    pub status: JobStatus,
    pub result: String,
}

impl From<Row<'_>> for JobStatusModel {
    fn from(value: Row) -> Self {
        JobStatusModel {
            id: String::from(value.get::<&str>("Id").unwrap()),
            result: String::from(value.get::<&str>("Result").unwrap_or_default()),
            status: JobStatus::from(value.get::<u32>("Status").unwrap()),
        }
    }
}

impl From<Job> for JobStatusModel {
    fn from(value: Job) -> Self {
        Self {
            id: value.id,
            result: value.result,
            status: value.status,
        }
    }
}
impl IntoBody for JobStatusModel {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

pub(crate) struct JobStatusList {
    status: Vec<JobStatusModel>,
}

impl From<Vec<JobStatusModel>> for JobStatusList {
    fn from(value: Vec<JobStatusModel>) -> Self {
        Self { status: value }
    }
}

impl IntoBody for JobStatusList {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self.status).unwrap()
    }
}
