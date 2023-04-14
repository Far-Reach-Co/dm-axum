use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// mod database;
// mod error;
// mod handler;

#[tokio::main]
async fn main() {
    // env
    let my_path = "./.env";
    dotenv::from_path(my_path).unwrap();
    // database
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
    // `POST /users` goes to `create_user`
    // .route("/users", post(create_user));

    const SOCKADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4000);
    println!("Serving at address: 'http://{}'", SOCKADDR);
    // serve
    tracing::debug!("listening on {}", SOCKADDR);
    axum::Server::bind(&SOCKADDR)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
