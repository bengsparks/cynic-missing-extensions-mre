#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Setup a server to "implement" our GraphQL Server
    let graphql = axum::Router::new().route("/graphql", axum::routing::get(graphql));
    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 1111);
    dbg!(&addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), graphql).await
}

#[axum::debug_handler]
async fn graphql() -> impl axum::response::IntoResponse {
    axum::Json(serde_json::json!({
        "errors": [
            {
                "message": "Unauthorized",
                "locations": null,
                "path": [
                    "helloWorld"
                ],
                "extensions": {
                    "code": 500
                }
            }
        ],
        "data": null
    }))
}
