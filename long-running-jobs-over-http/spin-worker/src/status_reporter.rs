use shared::JobStatus;
use spin_sdk::sqlite::{Connection, Value};

const SQL_UPDATE_JOB_STATUS: &str = "UPDATE Jobs SET  Status=?, Result=? WHERE Id=?";
pub fn report_job_status(id: &str, status: JobStatus, result: String) -> anyhow::Result<()> {
    let con = Connection::open_default()?;
    let params = [
        Value::Integer(status as i64),
        Value::Text(result),
        Value::Text(id.to_string()),
    ];
    con.execute(SQL_UPDATE_JOB_STATUS, &params)?;
    Ok(())
}
