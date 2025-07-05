use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};

use crate::{request_input::CreateWebsiteInput, request_output::CreateWebsiteOutput};
pub mod request_input;
pub mod request_output;
#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {

    let response = CreateWebsiteOutput {
        id: data.url
    };
    Json(response)
}

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("Website id is : {name}")
}



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello))
                                 .at("/website", post(create_website))
                                 .at("/website/:website_id", get(get_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}