use shared::{Job, JobStatus};
use spin_sdk::{
    mqtt::Connection,
    sqlite::{self, Value},
};

use crate::{
    config::Config,
    models::{CreateJobModel, JobStatusList, JobStatusModel},
};

const SQL_INSERT_JOB: &str = "INSERT INTO Jobs (Id, Input, Status) VALUES (?,?,?)";
const SQL_READ_JOB_STATUS: &str = "SELECT Id, Result, Status FROM Jobs WHERE Id=?";
const SQL_READ_ALL_JOBS_STATUS: &str = "SELECT Id, Result, Status FROM Jobs";
pub fn create_job(model: CreateJobModel) -> anyhow::Result<JobStatusModel> {
    let config = Config::load()?;

    let job = Job::from(model);
    let params = [
        Value::Text(job.id.clone()),
        Value::Text(job.input.clone()),
        Value::Integer(JobStatus::Pending as i64),
    ];
    let db = sqlite::Connection::open_default()?;
    match db.execute(SQL_INSERT_JOB, &params) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            ()
        }
    };
    let con = Connection::open(
        &config.get_connection_string(),
        &config.mqtt_username,
        &config.mqtt_password,
        config.mqtt_keep_alive,
    )?;
    let payload = serde_json::to_vec(&job)?;
    con.publish(
        &config.topic_name,
        &payload,
        spin_sdk::mqtt::Qos::ExactlyOnce,
    )?;
    let result = JobStatusModel::from(job);
    Ok(result)
}

pub fn read_job_status(id: String) -> anyhow::Result<Option<JobStatusModel>> {
    let con = sqlite::Connection::open_default()?;

    let params = [sqlite::Value::Text(id)];
    let row_set = con.execute(SQL_READ_JOB_STATUS, &params)?;
    let status: Vec<JobStatusModel> = row_set
        .rows()
        .map(|row| JobStatusModel::from(row))
        .collect();
    Ok(status.into_iter().next())
}

pub fn read_job_status_all() -> anyhow::Result<JobStatusList> {
    let con = sqlite::Connection::open_default()?;
    let params = [];
    let row_set = con.execute(SQL_READ_ALL_JOBS_STATUS, &params)?;
    let list: JobStatusList = row_set
        .rows()
        .map(|row| JobStatusModel::from(row))
        .collect::<Vec<JobStatusModel>>()
        .into();
    Ok(list)
}
