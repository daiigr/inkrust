use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use pulldown_cmark::{html, Options, Parser};

#[get("/")]
async fn markdown() -> impl Responder {
    let markdown_input = "# Hello, Markdown!";
    let parser = Parser::new_ext(markdown_input, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    HttpResponse::Ok().body(html_output)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(markdown)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
