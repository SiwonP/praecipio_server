mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "event")]
    pub struct Event {
        pub event_id: Option<i32>,
        pub event_name: String,
        pub event_location: String,
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "organize")]
    pub struct Organize {
        pub event_id: i32,
        pub organizer_id: i32,
    }

    #[derive(Deserialize, Serialize)]
    pub struct EventOrganized {
        pub event_name: String,
        pub event_location: String,
        pub organizer_id: i32,
    }
}

mod errors {
    
    use std::fmt::Display;

    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::From;
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }

    impl Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
            // match *self {
                // MyError::NotFound => "Not Found",   
                // MyError::PGError(_) => "PGError",
                // MyError::PGMError(_) => "PGMError",
                // MyError::PoolError(_) => "PoolError"
            // }
        }
    }

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => {
                    HttpResponse::InternalServerError().finish()
                },
            }
        }
    }
}

mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::{errors::MyError, models::Event, models::Organize};
    
    pub async fn create_event(client: &Client, event_info: Event) -> Result<Event, MyError> {
        let _stmt = "INSERT INTO event(event_name, event_location) VALUES($1, $2) RETURNING $table_fields;";
        let _stmt = _stmt.replace("$table_fields", &Event::sql_table_fields());
        // let statement = client.prepare(&_stmt).await.unwrap();
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.query(
            &statement, 
            &[
                &event_info.event_name, 
                &event_info.event_location
            ]
        )
        .await
        .map_err(MyError::PGError)?
        .iter()
        .map(|row| Event::from_row_ref(row).unwrap())
        .collect::<Vec<Event>>()
        .pop()
        .ok_or(MyError::NotFound)
    }

    pub async fn create_organize(client: &Client, organize_info: Organize) -> Result<Organize, MyError> {
        let _stmt = "INSERT INTO organize(organizer_id, event_id) VALUES($1,$2) RETURNING $table_fields;";
        let _stmt = _stmt.replace("$table_fields", &Organize::sql_table_fields());
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;
        
        client.query(
            &statement, 
            &[
                &organize_info.organizer_id, 
                &organize_info.event_id,
            ]
        )
        .await
        .map_err(MyError::PGError)?
        .iter()
        .map(|row| Organize::from_row_ref(row).unwrap())
        .collect::<Vec<Organize>>()
        .pop()
        .ok_or(MyError::NotFound)
    }
}

mod handlers {
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    use crate::{db, errors::MyError, models::{Event, EventOrganized, Organize}};

    pub async fn create_event(
        event_organized: web::Json<EventOrganized>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {

        let event_organized: EventOrganized = event_organized.into_inner();

        let event_info: Event = Event {
            event_id: None, 
            event_name: event_organized.event_name, 
            event_location: event_organized.event_location 
        };

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_event = db::create_event(&client, event_info).await?;

        let organize_info = Organize {
            event_id: new_event.event_id.unwrap(),
            organizer_id: event_organized.organizer_id,
        };

        let _new_organize = db::create_organize(&client, organize_info).await?;

        Ok(HttpResponse::Ok().json(new_event))
    }
}



// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{
//         http::{self, header::ContentType},
//         test,
//     };

//     #[actix_web::test]
//     async fn test_index_ok() {
//         let req = test::TestRequest::default()
//             .insert_header(ContentType::plaintext())
//             .to_http_request();
//         let resp = index(req).await;
//         assert_eq!(resp.status(), http::StatusCode::OK);
//     }

//     #[actix_web::test]
//     async fn test_index_not_ok() {
//         let req = test::TestRequest::default().to_http_request();
//         let resp = index(req).await;
//         assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
//     }
// }

use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::create_event;
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/event").route(web::post().to(create_event)))
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}