use actix_web::{get, App, HttpServer, Responder};
use actix_files::Files;
use rand::random;

#[get("/rnd")]
async fn rnd() -> impl Responder {
    let r = random::<usize>()%2;
    format!("{}", match r {
        0 => "No",
        1 => "Yes",
        _ => "Invalid"
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(rnd).service(Files::new("/", "./static/root/").index_file("index.html")))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
