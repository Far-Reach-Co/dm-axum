use dotenv;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::{hyper::StatusCode, Filter};

// mod db;
// mod error;
// mod handler;

#[tokio::main]
async fn main() {
    // env
    let my_path = "./.env";
    dotenv::from_path(my_path).unwrap();

    let env_key = "test";
    let env_value = dotenv::var(env_key).unwrap();
    println!("{}:{}", env_key, env_value);

    // health
    let health_status = warp::path("status").map(|| StatusCode::OK.to_string());

    // static
    let static_assets = warp::path::end().and(warp::fs::dir("./public"));

    let routes = warp::get().and(health_status.or(static_assets));

    const SOCKADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4000);
    println!("Serving at address: 'http://{}'", SOCKADDR);
    // serve
    warp::serve(routes).run(SOCKADDR).await;
}
