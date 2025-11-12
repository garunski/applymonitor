//! Comments state management

use crate::services::{
    comments_service::{Comment, CommentsService},
    error::ServiceError,
};
use dioxus::prelude::*;

/// Comments state containing signals for comments, loading, and error states
#[derive(Clone, Copy)]
pub struct CommentsState {
    pub comments: Signal<Vec<Comment>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<ServiceError>>,
}

/// Provide comments state context to the component tree
pub fn use_comments_provider() -> CommentsState {
    let comments = use_signal(Vec::<Comment>::new);
    let loading = use_signal(|| false);
    let error = use_signal(|| None::<ServiceError>);

    let state = CommentsState {
        comments,
        loading,
        error,
    };
    use_context_provider(|| state);
    state
}

/// Consume comments state context from the component tree
pub fn use_comments() -> CommentsState {
    use_context::<CommentsState>()
}

impl CommentsState {
    /// Fetch comments for a job
    pub fn fetch_comments_for_job(&self, job_id: String) {
        let mut comments = self.comments;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match CommentsService::fetch_comments(job_id).await {
                Ok(fetched_comments) => {
                    *comments.write() = fetched_comments;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Add a new comment
    pub fn add_comment(&self, job_id: String, content: String) {
        let mut comments = self.comments;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match CommentsService::create_comment(job_id, content).await {
                Ok(new_comment) => {
                    // Add comment to the beginning of the list
                    let mut comments_list = comments.read().clone();
                    comments_list.insert(0, new_comment);
                    *comments.write() = comments_list;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Set comments (used when loading from job details)
    pub fn set_comments(&self, new_comments: Vec<Comment>) {
        let mut comments = self.comments;
        *comments.write() = new_comments;
    }
}
