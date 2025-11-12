//! Jobs state management

use crate::services::{
    comments_service::Comment,
    emails_service::StoredEmail,
    error::ServiceError,
    jobs_service::{CreateJobRequest, Job, JobDetails, JobStatus, JobsService, UpdateJobRequest},
};
use crate::state::{CommentsState, EmailContactsState, EmailsState};
use dioxus::prelude::*;

/// Jobs state containing signals for jobs, loading, and error states
#[derive(Clone, Copy)]
pub struct JobsState {
    pub jobs: Signal<Vec<Job>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<ServiceError>>,
    pub selected_job: Signal<Option<JobDetails>>,
    pub created_job_id: Signal<Option<String>>,
    pub job_statuses: Signal<Vec<JobStatus>>,
}

/// Provide jobs state context to the component tree
pub fn use_jobs_provider() -> JobsState {
    let jobs = use_signal(Vec::<Job>::new);
    let loading = use_signal(|| false);
    let error = use_signal(|| None::<ServiceError>);
    let selected_job = use_signal(|| None::<JobDetails>);
    let created_job_id = use_signal(|| None::<String>);
    let job_statuses = use_signal(Vec::<JobStatus>::new);

    let state = JobsState {
        jobs,
        loading,
        error,
        selected_job,
        created_job_id,
        job_statuses,
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

    /// Fetch job details with related data and route to appropriate states
    pub fn fetch_job_details(
        &self,
        id: String,
        contacts_state: EmailContactsState,
        emails_state: EmailsState,
        comments_state: CommentsState,
    ) {
        let mut selected_job = self.selected_job;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match JobsService::fetch_job_details(id).await {
                Ok(api_response) => {
                    // Route contacts to EmailContactsState
                    contacts_state.set_contacts(api_response.contacts);

                    // Route emails to EmailsState (convert from Value to StoredEmail)
                    let emails: Vec<StoredEmail> = api_response
                        .emails
                        .iter()
                        .filter_map(|v| serde_json::from_value(v.clone()).ok())
                        .collect();
                    emails_state.set_emails(emails);

                    // Route comments to CommentsState (convert from Value to Comment)
                    let comments: Vec<Comment> = api_response
                        .comments
                        .iter()
                        .filter_map(|v| serde_json::from_value(v.clone()).ok())
                        .collect();
                    comments_state.set_comments(comments);

                    // Store only job and timeline_events in JobDetails
                    let job_details = JobDetails {
                        job: api_response.job,
                        timeline_events: api_response.timeline_events,
                    };
                    *selected_job.write() = Some(job_details);
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Update job description only
    pub fn update_job_description(&self, id: String, description: Option<String>) {
        let mut selected_job = self.selected_job;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match JobsService::update_job_description(id, description).await {
                Ok(updated_job) => {
                    // Update the job in selected_job if it exists
                    let mut current_details = selected_job.read().clone();
                    if let Some(ref mut details) = current_details {
                        details.job = updated_job;
                        *selected_job.write() = current_details;
                    }
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Set the created job ID signal (used for navigation after job creation)
    pub fn set_created_job_id(&self, id: String) {
        let mut created_job_id = self.created_job_id;
        *created_job_id.write() = Some(id);
    }

    /// Update job title only
    pub fn update_job_title(&self, id: String, title: String) {
        let mut selected_job = self.selected_job;
        let mut jobs = self.jobs;
        let mut loading = self.loading;
        let mut error = self.error;
        let id_clone = id.clone();

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            // Get current job to preserve other fields
            let current_job = if let Some(details) = selected_job.read().as_ref() {
                details.job.clone()
            } else {
                match JobsService::fetch_job(id_clone.clone()).await {
                    Ok(job) => job,
                    Err(e) => {
                        *error.write() = Some(e);
                        *loading.write() = false;
                        return;
                    }
                }
            };

            let request = UpdateJobRequest {
                title,
                company: current_job.company,
                location: current_job.location,
                status_id: current_job.status_id,
            };

            match JobsService::update_job(id_clone.clone(), request).await {
                Ok(updated_job) => {
                    // Update selected_job
                    let mut current_details = selected_job.read().clone();
                    if let Some(ref mut details) = current_details {
                        details.job = updated_job.clone();
                        *selected_job.write() = current_details;
                    }
                    // Update jobs list
                    let mut jobs_list = jobs.read().clone();
                    if let Some(index) = jobs_list.iter().position(|j| j.id == updated_job.id) {
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

    /// Update job company only
    pub fn update_job_company(&self, id: String, company: String) {
        let mut selected_job = self.selected_job;
        let mut jobs = self.jobs;
        let mut loading = self.loading;
        let mut error = self.error;
        let id_clone = id.clone();

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            // Get current job to preserve other fields
            let current_job = if let Some(details) = selected_job.read().as_ref() {
                details.job.clone()
            } else {
                match JobsService::fetch_job(id_clone.clone()).await {
                    Ok(job) => job,
                    Err(e) => {
                        *error.write() = Some(e);
                        *loading.write() = false;
                        return;
                    }
                }
            };

            let request = UpdateJobRequest {
                title: current_job.title,
                company,
                location: current_job.location,
                status_id: current_job.status_id,
            };

            match JobsService::update_job(id_clone.clone(), request).await {
                Ok(updated_job) => {
                    // Update selected_job
                    let mut current_details = selected_job.read().clone();
                    if let Some(ref mut details) = current_details {
                        details.job = updated_job.clone();
                        *selected_job.write() = current_details;
                    }
                    // Update jobs list
                    let mut jobs_list = jobs.read().clone();
                    if let Some(index) = jobs_list.iter().position(|j| j.id == updated_job.id) {
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

    /// Update job location only
    pub fn update_job_location(&self, id: String, location: Option<String>) {
        let mut selected_job = self.selected_job;
        let mut jobs = self.jobs;
        let mut loading = self.loading;
        let mut error = self.error;
        let id_clone = id.clone();

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            // Get current job to preserve other fields
            let current_job = if let Some(details) = selected_job.read().as_ref() {
                details.job.clone()
            } else {
                match JobsService::fetch_job(id_clone.clone()).await {
                    Ok(job) => job,
                    Err(e) => {
                        *error.write() = Some(e);
                        *loading.write() = false;
                        return;
                    }
                }
            };

            let request = UpdateJobRequest {
                title: current_job.title,
                company: current_job.company,
                location,
                status_id: current_job.status_id,
            };

            match JobsService::update_job(id_clone.clone(), request).await {
                Ok(updated_job) => {
                    // Update selected_job
                    let mut current_details = selected_job.read().clone();
                    if let Some(ref mut details) = current_details {
                        details.job = updated_job.clone();
                        *selected_job.write() = current_details;
                    }
                    // Update jobs list
                    let mut jobs_list = jobs.read().clone();
                    if let Some(index) = jobs_list.iter().position(|j| j.id == updated_job.id) {
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

    /// Fetch all job statuses
    pub fn fetch_job_statuses(&self) {
        let mut job_statuses = self.job_statuses;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match JobsService::fetch_job_statuses().await {
                Ok(statuses) => {
                    *job_statuses.write() = statuses;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Update job status only
    pub fn update_job_status(&self, id: String, status_id: i32) {
        let mut selected_job = self.selected_job;
        let mut jobs = self.jobs;
        let mut loading = self.loading;
        let mut error = self.error;
        let id_clone = id.clone();

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            // Get current job to preserve other fields
            let current_job = if let Some(details) = selected_job.read().as_ref() {
                details.job.clone()
            } else {
                match JobsService::fetch_job(id_clone.clone()).await {
                    Ok(job) => job,
                    Err(e) => {
                        *error.write() = Some(e);
                        *loading.write() = false;
                        return;
                    }
                }
            };

            let request = UpdateJobRequest {
                title: current_job.title,
                company: current_job.company,
                location: current_job.location,
                status_id: Some(status_id),
            };

            match JobsService::update_job(id_clone.clone(), request).await {
                Ok(updated_job) => {
                    // Update selected_job
                    let mut current_details = selected_job.read().clone();
                    if let Some(ref mut details) = current_details {
                        details.job = updated_job.clone();
                        *selected_job.write() = current_details;
                    }
                    // Update jobs list
                    let mut jobs_list = jobs.read().clone();
                    if let Some(index) = jobs_list.iter().position(|j| j.id == updated_job.id) {
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
}
