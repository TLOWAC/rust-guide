use actix_web::web::*;
use actix_web::*;

use serde::*;

#[derive(Serialize, Deserialize, Debug)]
struct User {
        name: String,
        age: u32,
        company: String,
}

#[get("/api/{name}")]
pub async fn get_user(name: Path<String>) -> Result<impl Responder> {
        let user = User {
                name: name.to_string(),
                age: 25,
                company: String::from("danbicorp"),
        };

        Ok(Json(user))
}
