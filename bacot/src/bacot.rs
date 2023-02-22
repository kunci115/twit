use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Bacots = Response<Bacot>;

use crate::constants::APPLICATION_JSON;
use crate::like::Like;
use crate::response::Response;


#[derive(Debug, Deserialize, Serialize)]
pub struct Bacot {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Bacot {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BacotRequest {
    pub message: Option<String>,
}

impl BacotRequest {
    pub fn to_bacot(&self) -> Option<Bacot> {
        match &self.message {
            Some(message) => Some(Bacot::new(message.to_string())),
            None => None,
        }
    }
}

/// list 50 last tweets
#[get("/bacots")]
pub async fn list() -> HttpResponse {
    let bacots = Bacots { results: vec![] };
    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(bacots)

}


/// create a tweet /bacots
#[post("/bacots")]
pub async fn create(bacot_req: Json<BacotRequest>) -> HttpResponse{
    HttpResponse::Created().content_type(APPLICATION_JSON).json(bacot_req.to_bacot())
}

/// find a tweet by its id /bacot/{id}
#[get("/bacots/{id}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {
    let found_bacot: Option<Bacot> = None;

    match found_bacot {
        Some(bacot) => HttpResponse::Ok()
        .content_type(APPLICATION_JSON).json(bacot),
        None=>HttpResponse::NoContent().content_type(APPLICATION_JSON)
        .await.unwrap(),
    }
}

/// delete a tweet by its id /bacots/{id}
#[delete("/bacots/{id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse {
    HttpResponse::NoContent().content_type(APPLICATION_JSON).await.unwrap()
}