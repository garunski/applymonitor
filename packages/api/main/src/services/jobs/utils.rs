//! Utility functions for jobs

use serde_json::Value;

/// Normalize job ID to string format
/// Handles both string and integer IDs from database
pub fn normalize_job_id(job: &mut Value) {
    if let Some(id_val) = job.get_mut("id") {
        if let Some(id_str) = id_val.as_str() {
            *id_val = Value::String(id_str.to_string());
        } else if let Some(id_int) = id_val.as_i64() {
            *id_val = Value::String(id_int.to_string());
        } else if let Some(id_int) = id_val.as_u64() {
            *id_val = Value::String(id_int.to_string());
        }
    }
}
