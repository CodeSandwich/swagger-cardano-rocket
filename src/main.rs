#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;

use rocket_contrib::json::Json;
use serde_json::Value as JsonValue;

fn main() {
    let routes = routes![
        next_update_v1,
        restart_node_v1,
    ];
    rocket::ignite()
        .mount("/api/", routes)
        .launch();
}

#[get("/v1/next-update")]
fn next_update_v1() -> Json<JsonValue> {
    Json(json!({
          "data": {
            "applicationName": "string",
            "version": 0
          },
          "meta": {
            "pagination": {}
          },
          "status": "success"
        }))
}

#[get("/v1/restart-node?<force_ntp_check>")]
fn restart_node_v1(force_ntp_check: Option<bool>) {
    println!("Restart! force_ntp_check = {}", force_ntp_check.unwrap_or(false));
}
