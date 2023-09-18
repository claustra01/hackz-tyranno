use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, Schema, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use objects::{QuerySchema, query::Query, mutation::Mutation};

mod objects;
mod db;

async fn index(schema: web::Data<QuerySchema>, req: GraphQLRequest) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
  Ok(HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  // DB接続テスト: 後で消す
  db::connection().expect("Failed to connect to database");

  let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

  println!("GraphiQL: http://localhost:8080");

  HttpServer::new(move || {
      App::new()
          .app_data(Data::new(schema.to_owned()))
          .service(web::resource("/").guard(guard::Post()).to(index))
          .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}