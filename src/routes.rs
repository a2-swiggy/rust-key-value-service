use actix_web::{web, HttpRequest, Responder};
use crate::AppState;
use crate::config_store::Request;

#[derive(Debug, Serialize, Deserialize)]
struct NewPair {
    name: String,
    value: String
}

pub fn get(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn add(app_state: web::Data<AppState>, pair: web::Json<NewPair>) -> impl Responder {
    app_state.store.clone().send(Request::AddPair(pair.name.to_owned(), pair.value.to_owned()));
}