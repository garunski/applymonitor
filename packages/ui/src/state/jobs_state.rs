//! Jobs state management

use crate::services::{
    error::ServiceError,
    jobs_service::{CreateJobRequest, Job, JobsService, UpdateJobRequest},
};
use dioxus::prelude::*;

/// Jobs state containing signals for jobs, loading, and error states
#[derive(Clone, Copy)]
pub struct JobsState {
    pub jobs: Signal<Vec<Job>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<ServiceError>>,
}

/// Provide jobs state context to the component tree
pub fn use_jobs_provider() -> JobsState {
    let jobs = use_signal(Vec::<Job>::new);
    let loading = use_signal(|| false);
    let error = use_signal(|| None::<ServiceError>);

    let state = JobsState {
        jobs,
        loading,
        error,
    };
    use_context_provider(|| state);
    state
}

/// Consume jobs state context from the component tree
pub fn use_jobs() -> JobsState {
    use_context::<JobsState>()
}

impl JobsState {
    /// Fetch all jobs
    pub fn fetch_jobs(&self) {
        let mut jobs = self.jobs;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match JobsService::fetch_jobs().await {
                Ok(fetched_jobs) => {
                    *jobs.write() = fetched_jobs;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Fetch a single job by ID
    pub fn fetch_job(&self, id: String) {
        let mut jobs = self.jobs;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match JobsService::fetch_job(id).await {
                Ok(job) => {
                    // Update the job in the list if it exists, otherwise add it
                    let mut jobs_list = jobs.read().clone();
                    if let Some(index) = jobs_list.iter().position(|j| j.id == job.id) {
                        jobs_list[index] = job;
                    } else {
                        jobs_list.push(job);
                    }
                    *jobs.write() = jobs_list;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Create a new job
    pub fn create_job(&self, job: CreateJobRequest) {
        let mut jobs = self.jobs;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match JobsService::create_job(job).await {
                Ok(created_job) => {
                    let mut jobs_list = jobs.read().clone();
                    jobs_list.push(created_job);
                    *jobs.write() = jobs_list;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Update an existing job
    pub fn update_job(&self, id: String, job: UpdateJobRequest) {
        let mut jobs = self.jobs;
        let mut loading = self.loading;
        let mut error = self.error;
        let id_clone = id.clone();

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match JobsService::update_job(id, job).await {
                Ok(updated_job) => {
                    let mut jobs_list = jobs.read().clone();
                    if let Some(index) = jobs_list
                        .iter()
                        .position(|j| j.id == Some(id_clone.clone()))
                    {
                        jobs_list[index] = updated_job;
                    }
                    *jobs.write() = jobs_list;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Delete a job
    pub fn delete_job(&self, id: String) {
        let mut jobs = self.jobs;
        let mut loading = self.loading;
        let mut error = self.error;
        let id_clone = id.clone();

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match JobsService::delete_job(id).await {
                Ok(_) => {
                    let mut jobs_list = jobs.read().clone();
                    jobs_list.retain(|j| j.id != Some(id_clone.clone()));
                    *jobs.write() = jobs_list;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }
}
