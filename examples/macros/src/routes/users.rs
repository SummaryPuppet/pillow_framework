use pillow::{
    http::{controller, Handler, Response},
    templates::{Context, Template},
};

#[controller(method = "GET", path = "/users/<id>")]
pub fn users() -> Response {
    println!("{:#?}", request.get_param("id"));

    let mut ctx = Context::new();

    ctx.insert("name", "SummaryPuppet");
    ctx.insert("id", &request.get_param("id"));

    Response::view(Template::Tera("users", "tera.html", ctx))
}
