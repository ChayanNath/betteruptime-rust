
use std::sync::{Arc, Mutex};
use poem::{
    get, handler, listener::TcpListener, post, web::{Path}, EndpointExt, Route, Server
};
use store::store::Store;

use crate::routes::{user::{sign_in, sign_up}, website::{create_website, get_website}};


pub mod request_input;
pub mod request_output;
pub mod routes;

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::new().unwrap()));
    let app = Route::new().at("/hello/:name", get(hello))
                        .at("/website", post(create_website))
                        .at("/website/:website_id", get(get_website))
                        .at("/user/signup", post(sign_up))
                        .at("/user/signin", post(sign_in))
                        .data(s);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("better-uptime")
        .run(app)
        .await
}