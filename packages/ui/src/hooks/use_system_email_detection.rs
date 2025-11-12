//! Hook for system email detection

use crate::services::email_contacts_service::EmailContactsService;
use crate::services::jobs_service::EmailContact;
use dioxus::prelude::*;

/// Hook return value for system email detection
pub struct SystemEmailDetection {
    pub is_system_detected: Signal<bool>,
    pub checking: Signal<bool>,
}

/// Hook to detect if a contact email is a system email
/// Prevents infinite loops by tracking email address and only checking when it changes
pub fn use_system_email_detection(contact: Signal<Option<EmailContact>>) -> SystemEmailDetection {
    let is_system_detected = use_signal(|| false);
    let checking = use_signal(|| false);
    let last_checked_email = use_signal(|| None::<String>);

    use_effect({
        let contact_signal = contact;
        let mut detected_signal = is_system_detected;
        let mut checking_signal = checking;
        let mut last_email_signal = last_checked_email;
        move || {
            let contact_opt = contact_signal.read().clone();
            if let Some(ref c) = contact_opt {
                let current_email = c.email.clone();
                let last_email = last_email_signal.read().clone();

                // Only check if email changed or if we haven't checked yet
                if last_email.as_ref() != Some(&current_email) {
                    *last_email_signal.write() = Some(current_email.clone());

                    // Use is_system_detected from contact if available
                    if c.is_system_detected {
                        *detected_signal.write() = true;
                        *checking_signal.write() = false;
                    } else if !checking_signal() {
                        // Check if not already checking
                        let email = current_email.clone();
                        spawn(async move {
                            *checking_signal.write() = true;
                            match EmailContactsService::check_system_email(&email).await {
                                Ok(detected) => {
                                    *detected_signal.write() = detected;
                                }
                                Err(_) => {
                                    *detected_signal.write() = false;
                                }
                            }
                            *checking_signal.write() = false;
                        });
                    }
                }
            } else {
                *detected_signal.write() = false;
                *checking_signal.write() = false;
                *last_email_signal.write() = None;
            }
        }
    });

    SystemEmailDetection {
        is_system_detected,
        checking,
    }
}
