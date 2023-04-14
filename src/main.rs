use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .with_state(pool);
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

async fn root(State(pool): State<PgPool>) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    sqlx::query_scalar("select * from public.\"User\";")
        .fetch_all(&pool)
        .await
        .map(|users| {
            for user in users.iter() {
                println!("{:?}", user);
            }
            Json(users)
        })
        .map_err(internal_error)
}

struct User {
    id: i32,
    email: String,
    password: String,
    name: String,
    phone: String,
    is_pro: bool,
    username: String,
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
