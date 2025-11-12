//! This crate contains all shared UI for the workspace.

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod table;
pub use table::{Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow};

mod badge;
pub use badge::Badge;

mod jobs_list;
pub use jobs_list::JobsList;

mod job_form;
pub use job_form::JobForm;

mod dashboard;
pub use dashboard::DashboardContent;

mod emails_list;
pub use emails_list::EmailsList;

mod email_slideout;
pub use email_slideout::EmailSlideout;

mod job_select_dialog;
pub use job_select_dialog::JobSelectDialog;

mod sidebar;
pub use sidebar::{SidebarLayout, SidebarLayoutProps};

mod timeline;
pub use timeline::Timeline;

mod comment_form;
pub use comment_form::CommentForm;

mod job_details;
pub use job_details::JobDetails;

mod job_details_components;

mod email_contact_card;
pub use email_contact_card::EmailContactCard;

mod email_contact_slideout;
pub use email_contact_slideout::EmailContactSlideout;

pub mod services;
pub mod state;
pub mod utils;

pub mod components;
pub mod hooks;

// Re-export commonly used components
pub use components::login_button::LoginButton;
pub use components::login_page::LoginPage;
pub use components::sidebar_nav::{SidebarNav, SidebarNavItem};
pub use components::user_profile::UserProfile;
pub use hooks::use_system_email_detection;
pub use state::{use_auth, use_auth_provider};
