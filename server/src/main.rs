use axum::{
    routing::get_service,
    Router,
    serve
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new().fallback_service(ServeDir::new("frontend/dist/").append_index_html_on_directories(true));
    let binding = format!("{}:{}", "0.0.0.0", 3000);

    // Make the TcpListener, if that is successful, try `serve()` the Axum app
    match tokio::net::TcpListener::bind(&binding).await {
        Ok(listener) => {

            println!("Listener initialised on {}", &binding);
            match serve(listener, app).await {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("Error starting Axum server: {}", err.to_string());
                }
            };

        },
        Err(err) => {
            eprintln!("Error binding listener: {}", err.to_string());
        }
    };
}

