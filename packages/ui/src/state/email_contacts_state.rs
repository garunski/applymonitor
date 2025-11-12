//! Email contacts state management

use crate::services::{
    email_contacts_service::EmailContactsService, error::ServiceError, jobs_service::EmailContact,
};
use dioxus::prelude::*;

/// Email contacts state containing signals for contacts, loading, error, and selected contact
#[derive(Clone, Copy)]
pub struct EmailContactsState {
    pub contacts: Signal<Vec<EmailContact>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<ServiceError>>,
    pub selected_contact: Signal<Option<EmailContact>>,
}

/// Provide email contacts state context to the component tree
pub fn use_email_contacts_provider() -> EmailContactsState {
    let contacts = use_signal(Vec::<EmailContact>::new);
    let loading = use_signal(|| false);
    let error = use_signal(|| None::<ServiceError>);
    let selected_contact = use_signal(|| None::<EmailContact>);

    let state = EmailContactsState {
        contacts,
        loading,
        error,
        selected_contact,
    };
    use_context_provider(|| state);
    state
}

/// Consume email contacts state context from the component tree
pub fn use_email_contacts() -> EmailContactsState {
    use_context::<EmailContactsState>()
}

impl EmailContactsState {
    /// Fetch contacts for a job
    pub fn fetch_contacts_for_job(&self, job_id: String) {
        let mut contacts = self.contacts;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match EmailContactsService::get_contacts_for_job(&job_id).await {
                Ok(fetched_contacts) => {
                    *contacts.write() = fetched_contacts;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Select a contact (opens slideout)
    pub fn select_contact(&self, contact: EmailContact) {
        let mut selected = self.selected_contact;
        *selected.write() = Some(contact);
    }

    /// Clear selected contact (closes slideout)
    pub fn clear_selected(&self) {
        let mut selected = self.selected_contact;
        *selected.write() = None;
    }

    /// Update a contact
    pub fn update_contact(
        &self,
        email: String,
        name: Option<String>,
        linkedin: Option<String>,
        website: Option<String>,
    ) {
        let mut contacts = self.contacts;
        let mut loading = self.loading;
        let mut error = self.error;
        let mut selected_contact = self.selected_contact;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match EmailContactsService::update_contact(
                &email,
                name.clone(),
                linkedin.clone(),
                website.clone(),
            )
            .await
            {
                Ok(updated_contact) => {
                    // Update the contact in the list
                    let mut contacts_list = contacts.read().clone();
                    if let Some(index) = contacts_list
                        .iter()
                        .position(|c| c.email == updated_contact.email)
                    {
                        contacts_list[index] = updated_contact.clone();
                    } else {
                        contacts_list.push(updated_contact.clone());
                    }
                    *contacts.write() = contacts_list;

                    // Update selected contact if it's the same one
                    let selected_email = selected_contact.read().as_ref().map(|c| c.email.clone());
                    if let Some(sel_email) = selected_email {
                        if sel_email == updated_contact.email {
                            *selected_contact.write() = Some(updated_contact);
                        }
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

    /// Set contacts (used when loading from job details)
    pub fn set_contacts(&self, new_contacts: Vec<EmailContact>) {
        let mut contacts = self.contacts;
        *contacts.write() = new_contacts;
    }

    /// Convert a system email contact to a user-saved contact
    pub fn convert_to_user_contact(&self, email: String) {
        let mut contacts = self.contacts;
        let mut loading = self.loading;
        let mut error = self.error;
        let mut selected_contact = self.selected_contact;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match EmailContactsService::convert_to_user_contact(&email).await {
                Ok(updated_contact) => {
                    // Update the contact in the list
                    let mut contacts_list = contacts.read().clone();
                    if let Some(index) = contacts_list
                        .iter()
                        .position(|c| c.email == updated_contact.email)
                    {
                        contacts_list[index] = updated_contact.clone();
                    } else {
                        contacts_list.push(updated_contact.clone());
                    }
                    *contacts.write() = contacts_list;

                    // Update selected contact if it's the same one
                    let selected_email = selected_contact.read().as_ref().map(|c| c.email.clone());
                    if let Some(sel_email) = selected_email {
                        if sel_email == updated_contact.email {
                            *selected_contact.write() = Some(updated_contact);
                        }
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
}
