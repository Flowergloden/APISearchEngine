use std::{fs::File, io::Read};

use rocket::{get, launch, routes, serde::json::Json};
use server::{
    cors::CORS,
    parser::TagParser,
    response::{Body, Item},
};

#[get("/search/<mode>/<keyword>")]
fn search(mode: &str, keyword: &str) -> Json<Body> {
    let items = vec![Item::mock("asdf"), Item::mock("hjkl")];
    let body = Body { items };
    Json(body)
}

#[launch]
fn start() -> _ {
    let mut file = File::open("cppreference-doxygen-web.tag.xml").unwrap();
    // let mut file = File::open("test.tag.xml").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let content = TagParser.parse_doc(&input).unwrap();
    println!("{:?}", content);

    rocket::build().attach(CORS).mount("/", routes![search])
}
