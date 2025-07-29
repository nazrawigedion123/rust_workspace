use actix_files::{NamedFile,Files};
use actix_web::{App, HttpServer, Result, web};

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listen on port 8080");
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "static").show_files_listing())
            .service(Files::new("/image", "image").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
