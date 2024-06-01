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
    let input = include_str!("cppreference-doxygen-web.tag.xml");
    let content = TagParser.parse_doc(&input).unwrap();

    rocket::build().attach(CORS).mount("/", routes![search])
}
