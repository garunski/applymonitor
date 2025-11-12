//! Job details components

mod company_location;
mod description;
mod details_tab;
mod emails_tab;
mod header;

pub use company_location::{CompanyField, LocationField};
pub use description::DescriptionField;
pub use details_tab::DetailsTab;
pub use emails_tab::EmailsTab;
pub use header::JobDetailsHeader;
