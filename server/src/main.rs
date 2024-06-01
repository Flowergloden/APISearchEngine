use rocket::{get, launch, routes, serde::json::Json, State};
use server::{
    cors::CORS,
    parser::{TagContent, TagParser},
    response::Body,
};

#[get("/search/<mode>/<keyword>")]
fn search(mode: &str, keyword: &str, content: &State<TagContent>) -> Json<Body> {
    let items = match mode {
        "name" => content
            .search_in_name(keyword)
            .into_iter()
            .map(|x| x.into())
            .collect(),
        "para" => content
            .search_in_para(keyword)
            .into_iter()
            .map(|x| x.into())
            .collect(),
        "rt" => content
            .search_in_rt(keyword)
            .into_iter()
            .map(|x| x.into())
            .collect(),
        _ => vec![],
    };
    let body = Body { items };
    Json(body)
}

#[launch]
fn start() -> _ {
    let input = include_str!("cppreference-doxygen-web.tag.xml");
    let content = TagParser.parse_doc(input).unwrap();

    rocket::build()
        .manage(content)
        .attach(CORS)
        .mount("/", routes![search])
}
