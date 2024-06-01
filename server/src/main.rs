use std::{fs::File, io::BufReader};
use rocket::{get, launch, routes, serde::json::Json};
use server::{
    cors::CORS,
    data::TagData,
    response::{Body, Item},
};
use xml::reader::{EventReader, XmlEvent};

#[get("/search/<mode>/<keyword>")]
fn search(mode: &str, keyword: &str) -> Json<Body> {
    let items = vec![Item::mock("asdf"), Item::mock("hjkl")];
    let body = Body { items };
    Json(body)
}

#[launch]
fn start() -> _ {
    let file = File::open("test.tag.xml").unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    rocket::build().attach(CORS).mount("/", routes![search])
}
