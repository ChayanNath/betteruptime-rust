use std::sync::{Arc, Mutex};

use poem::{
    get, handler, listener::TcpListener, post, web::{Data, Json, Path}, EndpointExt, Route, Server
};
use store::store::Store;


use crate::{request_input::{CreateUserInput, CreateWebsiteInput}, request_output::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SignInOutput}};
pub mod request_input;
pub mod request_output;
#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.create_website(String::from("dd020379-1e62-44b2-8a3d-c4e17c30d044"), data.url).unwrap();
    let response = CreateWebsiteOutput {
        id: website.id
    };
    Json(response)
}
 
#[handler]
fn get_website(Path(id): Path<String>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id).unwrap();
    Json(GetWebsiteOutput { url: website.url })
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateUserOutput> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).unwrap();
    let response = CreateUserOutput {
        id
    };
    Json(response)
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<SignInOutput> {
    let mut locked_s = s.lock().unwrap();
    let _exists = locked_s.sign_in(data.username, data.password).unwrap();
    let response = SignInOutput { jwt: String::from("Chayan") };
    Json(response)
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
        .name("hello-world")
        .run(app)
        .await
}