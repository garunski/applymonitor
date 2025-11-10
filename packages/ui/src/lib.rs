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

pub mod services;
pub mod state;

pub mod components;
