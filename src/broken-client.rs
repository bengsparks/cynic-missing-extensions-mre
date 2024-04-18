use cynic::{http::ReqwestExt as _, QueryBuilder};
mod query;
mod schema;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let operation = query::HelloWorld::build(());
    let response = client
        .get("http://127.0.0.1:1111/graphql")
        .run_graphql(operation)
        .await;
    dbg!(&response);
}
