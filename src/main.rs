use crate::database::Database;
use crate::query_engine::Query;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::routing::post;
use axum::Router;

mod database;
mod query_engine;
mod shape_service;

async fn graphql_handler(request: GraphQLRequest) -> GraphQLResponse {
    let query = Query { database: Database };

    let schema = Schema::new(query, EmptyMutation, EmptySubscription);

    let res = schema.execute(request.into_inner()).await;

    res.into()
}

// NA
// - resolve error
// The default runtime flavor is `multi_thread`, but the `rt-multi-thread` feature is disabled.
// - https://github.com/tokio-rs/tokio/discussions/4718
// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new().route("/shapes", post(graphql_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on port 3000.");
    axum::serve(listener, app).await.unwrap();
}
