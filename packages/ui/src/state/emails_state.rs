//! Emails state management

use crate::services::{
    emails_service::{AssignJobRequest, EmailsService, StoredEmail},
    error::ServiceError,
    jobs_service::CreateJobRequest,
};
use crate::state::use_jobs;
use dioxus::prelude::*;

/// Emails state containing signals for emails, loading, error, and selected email
#[derive(Clone, Copy)]
pub struct EmailsState {
    pub emails: Signal<Vec<StoredEmail>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<ServiceError>>,
    pub selected_email: Signal<Option<StoredEmail>>,
}

/// Provide emails state context to the component tree
pub fn use_emails_provider() -> EmailsState {
    let emails = use_signal(Vec::<StoredEmail>::new);
    let loading = use_signal(|| false);
    let error = use_signal(|| None::<ServiceError>);
    let selected_email = use_signal(|| None::<StoredEmail>);

    let state = EmailsState {
        emails,
        loading,
        error,
        selected_email,
    };
    use_context_provider(|| state);
    state
}

/// Consume emails state context from the component tree
pub fn use_emails() -> EmailsState {
    use_context::<EmailsState>()
}

impl EmailsState {
    /// Fetch all emails
    pub fn fetch_emails(&self, limit: Option<usize>, offset: Option<usize>) {
        let mut emails = self.emails;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match EmailsService::list_emails(limit, offset).await {
                Ok(fetched_emails) => {
                    *emails.write() = fetched_emails;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Fetch a single email by ID
    pub fn fetch_email(&self, id: String) {
        let mut emails = self.emails;
        let mut loading = self.loading;
        let mut error = self.error;
        let mut selected_email = self.selected_email;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match EmailsService::get_email(id.clone()).await {
                Ok(email) => {
                    // Update the email in the list if it exists, otherwise add it
                    let mut emails_list = emails.read().clone();
                    if let Some(index) = emails_list
                        .iter()
                        .position(|e| e.gmail_id == email.gmail_id)
                    {
                        emails_list[index] = email.clone();
                    } else {
                        emails_list.push(email.clone());
                    }
                    *emails.write() = emails_list;
                    *selected_email.write() = Some(email);
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Select an email (opens slideout)
    pub fn select_email(&self, email: StoredEmail) {
        let mut selected = self.selected_email;
        *selected.write() = Some(email);
    }

    /// Clear selected email (closes slideout)
    pub fn clear_selected(&self) {
        let mut selected = self.selected_email;
        *selected.write() = None;
    }

    /// Assign email to job (create new job from email)
    pub fn assign_to_job(&self, gmail_id: String, create_job: CreateJobRequest) {
        let mut loading = self.loading;
        let mut error = self.error;
        let jobs_state = use_jobs();

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            let request = AssignJobRequest {
                job_id: None,
                create_job: Some(crate::services::emails_service::CreateJobFromEmail {
                    title: create_job.title,
                    company: create_job.company,
                    location: create_job.location,
                    status: Some(create_job.status),
                }),
            };

            match EmailsService::assign_email_to_job(gmail_id, request).await {
                Ok(response) => {
                    *error.write() = None;
                    // Set created job ID for navigation
                    jobs_state.set_created_job_id(response.job_id);
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Assign email to existing job
    pub fn assign_to_existing_job(&self, gmail_id: String, job_id: String) {
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            let request = AssignJobRequest {
                job_id: Some(job_id),
                create_job: None,
            };

            match EmailsService::assign_email_to_job(gmail_id, request).await {
                Ok(_) => {
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
