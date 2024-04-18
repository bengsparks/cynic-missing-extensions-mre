# Replacing `run_graphql` with `run_graphql_with_ext` in order to extract extensions 

## Start "GraphQL" server at http://127.0.0.1:1111

`cargo run --bin server`

## Use provided `ReqwestExt::run_graphql` trait to query the server

`cargo run --bin broken-client`

```rust
[src/broken-client.rs:14:5] &response = Ok(
    GraphQlResponse {
        data: None,
        errors: Some(
            [
                GraphQlError {
                    message: "Unauthorized",
                    locations: None,
                    path: Some(
                        [
                            Field(
                                "helloWorld",
                            ),
                        ],
                    ),
                    extensions: Some(
                        IgnoredAny,
                    ),
                },
            ],
        ),
    },
)
```

## Use manual deserialisation in implementation based on `TweakedReqwestExt::run_graphql_with_ext`

`cargo run --bin working-client`


```rust
[src/working-client.rs:19:5] &response = Ok(
    GraphQlResponse {
        data: None,
        errors: Some(
            [
                GraphQlError {
                    message: "Unauthorized",
                    locations: None,
                    path: Some(
                        [
                            Field(
                                "helloWorld",
                            ),
                        ],
                    ),
                    extensions: Some(
                        Extension {
                            code: 500,
                        },
                    ),
                },
            ],
        ),
    },
)
```