use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::*;
use async_graphql_poem::GraphQL;
use dotenv::dotenv;
use info_api::query::*;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
    let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)));
    println!("GraphiQL IDE: http://localhost:8080");
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
        .unwrap();
}
