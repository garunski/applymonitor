//! Job-related services

pub mod contacts;
pub mod create;
pub mod delete;
pub mod details;
pub mod people;
pub mod read;
pub mod timeline;
pub mod types;
pub mod update;
pub mod utils;

pub use contacts::process_contacts_for_job;
pub use create::create_job;
pub use delete::delete_job;
pub use details::get_job_details_data;
pub use people::extract_people_from_emails;
pub use read::{get_job, list_jobs};
pub use timeline::build_timeline_events;
pub use types::{Job, JobStatus};
pub use update::update_job;
pub use utils::normalize_job_id;
