use rocket::Route;

use crate::{ServerState, db::db_health};

#[get("/healthz")]
pub async fn health(state: &ServerState) -> Result<String, String> {
    let state = state.get().await;
    let db = state.db().await;

    let res = db_health(db);

    if res {
        Ok("OK".to_string())
    } else {
        Err("Not healthy".to_string())
    }
}

pub fn api() -> Vec<Route> {
    routes![health]
}
