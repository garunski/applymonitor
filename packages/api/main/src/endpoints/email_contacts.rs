use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::db::email_contacts::{get_contact, get_contacts_for_job, update_contact};
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateContactRequest {
    pub name: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
}

pub async fn handler(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = get_d1(&ctx.env)?;
    let user_id = require_auth(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let method = req.method();
    let url = req.url()?;
    let query_params = url
        .query_pairs()
        .collect::<std::collections::HashMap<_, _>>();

    match method {
        Method::Get => {
            // GET /email-contacts?job_id={id} - Get contacts for a job
            if let Some(job_id) = query_params.get("job_id") {
                match get_contacts_for_job(&db, job_id, &user_id).await {
                    Ok(contacts) => {
                        let contacts_json: Vec<serde_json::Value> = contacts
                            .iter()
                            .map(|c| {
                                serde_json::json!({
                                    "email": c.email,
                                    "user_id": c.user_id,
                                    "name": c.name,
                                    "linkedin": c.linkedin,
                                    "website": c.website,
                                    "is_system": c.is_system,
                                })
                            })
                            .collect();
                        return Response::from_json(&contacts_json);
                    }
                    Err(e) => {
                        return Response::error(format!("Failed to fetch contacts: {}", e), 500);
                    }
                }
            }

            // GET /email-contacts/{email} - Get specific contact
            if let Some(email) = ctx.param("email") {
                match get_contact(&db, email, &user_id).await {
                    Ok(Some(c)) => {
                        let contact_json = serde_json::json!({
                            "email": c.email,
                            "user_id": c.user_id,
                            "name": c.name,
                            "linkedin": c.linkedin,
                            "website": c.website,
                            "is_system": c.is_system,
                        });
                        return Response::from_json(&contact_json);
                    }
                    Ok(None) => {
                        return Response::error("Contact not found", 404);
                    }
                    Err(e) => {
                        return Response::error(format!("Failed to fetch contact: {}", e), 500);
                    }
                }
            }

            Response::error(
                "Missing job_id query parameter or email path parameter",
                400,
            )
        }
        Method::Put => {
            // PUT /email-contacts/{email} - Update contact
            if let Some(email) = ctx.param("email") {
                let body = req.json::<UpdateContactRequest>().await?;
                match update_contact(
                    &db,
                    email,
                    &user_id,
                    body.name.as_deref(),
                    body.linkedin.as_deref(),
                    body.website.as_deref(),
                )
                .await
                {
                    Ok(contact) => {
                        let contact_json = serde_json::json!({
                            "email": contact.email,
                            "user_id": contact.user_id,
                            "name": contact.name,
                            "linkedin": contact.linkedin,
                            "website": contact.website,
                            "is_system": contact.is_system,
                        });
                        return Response::from_json(&contact_json);
                    }
                    Err(e) => {
                        return Response::error(format!("Failed to update contact: {}", e), 500);
                    }
                }
            }

            Response::error("Missing email path parameter", 400)
        }
        _ => Response::error("Method not allowed", 405),
    }
}
