use axum::{
    routing::{get,post},
    Router,
    Json,};
use serde::{Deserialize,Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app=Router::new()
        .route("/",get(homepage))
        .route("/about",get(about))
        .route("/contact",post(contact) );
    let addr= SocketAddr::from(([127,0,0,1],3000));

    println!("server running at http://{}",addr);


    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

// Homeage handler
async fn homepage()-> &'static str{
    "welcome to my rust website"
}

// about page handler
async fn about()-> &'static str {
    "this is the about page of the rust website"
}
#[derive(Deserialize)]
struct  ContactForm {
    name :String,
    message: String,
}

#[derive(Serialize)]
struct ResponseMessage{
    status :String,
    message:String,
}

async fn contact(Json(payload):Json<ContactForm>)->Json<ResponseMessage>{
Json(ResponseMessage{
    status:"success".to_string(),
    message:format!("Thanks for reaching out,{}!",payload.name),}
    )
}





