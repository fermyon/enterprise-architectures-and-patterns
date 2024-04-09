use anyhow::bail;
use chrono::{DateTime, Utc};

use shared::Job;
use spin_mqtt_sdk::{mqtt_component, Payload};

use status_reporter::report_job_status;
use std::{self, thread::sleep, time::Duration};

mod status_reporter;

const MAGIC_WORD: &str = "foobar";

#[mqtt_component]
fn handle_message(payload: Payload) -> anyhow::Result<()> {
    log("Received a Job request");
    let req = serde_json::from_slice::<Job>(&payload)?;
    report_job_status(&req.id, shared::JobStatus::Running, String::new())?;
    match simulate_job(req.input) {
        Ok(result) => {
            report_job_status(&req.id, shared::JobStatus::Succeeded, result)?;
            log("Succeeded")
        }
        Err(e) => {
            report_job_status(&req.id, shared::JobStatus::Failed, e.to_string())?;
            log("Failed.")
        }
    }
    log("Job processed.");
    Ok(())
}

fn simulate_job(input: String) -> anyhow::Result<String> {
    sleep(Duration::from_secs(60));
    if input.to_lowercase().eq(MAGIC_WORD) {
        bail!("Received '{}' as input", MAGIC_WORD)
    }
    Ok(format!("{}!", input))
}

fn log(msg: &str) {
    let dt: DateTime<Utc> = std::time::SystemTime::now().into();
    let dt = dt.format("%H:%M:%S.%f").to_string();
    println!("{:?}: {}", dt, msg);
}
