use cynic::{http::CynicReqwestError, GraphQlResponse, Operation, QueryBuilder};
mod query;
mod schema;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Extension {
    code: std::num::NonZeroU16,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let operation = query::HelloWorld::build(());
    let response: Result<GraphQlResponse<query::HelloWorld, Extension>, CynicReqwestError> = client
        .get("http://127.0.0.1:1111/graphql")
        .run_graphql_with_ext(operation)
        .await;
    dbg!(&response);
}

type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

pub trait TweakedReqwestExt {
    /// Runs a GraphQL query with the parameters in RequestBuilder, deserializes
    /// the and returns the result.
    fn run_graphql_with_ext<ResponseData, Vars, ErrorExtensions /*  <-- Add this */>(
        self,
        operation: Operation<ResponseData, Vars>,
    ) -> BoxFuture<'static, Result<GraphQlResponse<ResponseData, ErrorExtensions>, CynicReqwestError>>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
        ErrorExtensions: serde::de::DeserializeOwned;
}

impl TweakedReqwestExt for reqwest::RequestBuilder {
    fn run_graphql_with_ext<ResponseData, Vars, ErrorExtensions>(
        self,
        operation: Operation<ResponseData, Vars>,
    ) -> BoxFuture<'static, Result<GraphQlResponse<ResponseData, ErrorExtensions>, CynicReqwestError>>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
        ErrorExtensions: serde::de::DeserializeOwned, /*  <-- Add this */
    {
        let builder = self.json(&operation);
        Box::pin(async move {
            match builder.send().await {
                Ok(response) => {
                    let status = response.status();
                    if !status.is_success() {
                        let body_string = response.text().await?;

                        match serde_json::from_str::<GraphQlResponse<ResponseData, ErrorExtensions /*  <-- Add this */>>(
                            &body_string,
                        ) {
                            Ok(response) => return Ok(response),
                            Err(_) => {
                                return Err(CynicReqwestError::ErrorResponse(status, body_string));
                            }
                        };
                    }

                    response
                        .json::<GraphQlResponse<ResponseData, ErrorExtensions /*  <-- Add this */>>()
                        .await
                        .map_err(CynicReqwestError::ReqwestError)
                }
                Err(e) => Err(CynicReqwestError::ReqwestError(e)),
            }
        })
    }
}
