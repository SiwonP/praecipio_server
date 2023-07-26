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
        pub event_description: String,
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "person")]
    pub struct Person {
        pub person_id: Option<i32>,
        pub person_name: String,
        pub planner_id : Option<i32>,
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "planner")]
    pub struct Planner {
        pub planner_id: i32,
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "plan")]
    pub struct Plan {
        pub plan_id: Option<i32>,
        pub event_id: i32,
        pub planner_id: i32,
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "affiliation")]
    pub struct Affiliation {
        pub affiliation_id: Option<i32>,
        pub person_id: i32,
        pub organization_id: i32,
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "participation")]
    pub struct Participation {
        pub participation_id: Option<i32>,
        pub event_id: i32,
        pub person_id: i32,
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "organization")]
    pub struct Organization {
        pub organization_id: Option<i32>,
        pub organization_name: String,
        pub planner_id: Option<i32>,
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
                MyError::PGError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                },
                _ => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::{errors::MyError, models::{Event, Person, Plan, Planner, Affiliation, Organization}};
    
    pub async fn create_event(client: &Client, event_info: Event) -> Result<Event, MyError> {
        let _stmt = "INSERT INTO event(event_name, event_location, event_description) VALUES($1, $2, $3) RETURNING $table_fields;";
        let _stmt = _stmt.replace("$table_fields", &Event::sql_table_fields());
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.query(
            &statement, 
            &[
                &event_info.event_name, 
                &event_info.event_location,
                &event_info.event_description,
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

    pub async fn modify_event(client: &Client, event_info: Event) -> Result<Event, MyError> {
        let _stmt = "UPDATE event SET event_name = $1, event_description = $2 where event_id=$3 RETURNING $table_fields;";
        let _stmt = _stmt.replace("$table_fields", &Event::sql_table_fields());

        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.query(
            &statement,
            &[
                &event_info.event_name,
                &event_info.event_description,
                &event_info.event_id.unwrap().to_string(),
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

    pub async fn get_events(client: &Client, person_info: Person) -> Result<Vec<Event>, MyError> {
        let _stmt = "SELECT event.event_id, event.event_name, event.event_description, event.event_location FROM event
        JOIN plan ON event.event_id = plan.event_id 
        JOIN planner ON plan.planner_id = planner.planner_id
        JOIN person ON planner.planner_id = person.planner_id
        WHERE person.person_id = $1;";
        let _stmt = _stmt.to_string();
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        Ok(client.query(
            &statement, 
            &[
                &person_info.person_id, 
            ]
        )
        .await
        .map_err(MyError::PGError)?
        .iter()
        .map(|row| Event::from_row_ref(row).unwrap())
        .collect::<Vec<Event>>())

    }

    pub async fn delete_event(client: &Client, event_info: Event) -> Result<u64, MyError> {
        let _stmt = "DELETE FROM event WHERE event_id = $1;";
        let _stmt = _stmt.to_string();
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.execute(
            &statement,
            &[
                &event_info.event_id.unwrap().to_string(),
            ]
        )
        .await
        .map_err(MyError::PGError)

    }

    pub async fn create_plan(client: &Client, plan_info: Plan) -> Result<Plan, MyError> {
        let _stmt = "INSERT INTO plan(planner_id, event_id) VALUES($1,$2) RETURNING $table_fields;";
        let _stmt = _stmt.replace("$table_fields", &Plan::sql_table_fields());
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;
        
        client.query(
            &statement, 
            &[
                &plan_info.planner_id, 
                &plan_info.event_id,
            ]
        )
        .await
        .map_err(MyError::PGError)?
        .iter()
        .map(|row| Plan::from_row_ref(row).unwrap())
        .collect::<Vec<Plan>>()
        .pop()
        .ok_or(MyError::NotFound)
    }

    pub async fn delete_plan(client: &Client, plan_info: Plan) -> Result<u64, MyError> {
        let _stmt = "DELETE FROM plan WHERE plan_id = $1;";
        let _stmt = _stmt.to_string();
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.execute(
            &statement,
            &[
                &plan_info.plan_id.unwrap().to_string(),
            ]
        )
        .await
        .map_err(MyError::PGError)
    }

    pub async fn create_planner(client: &Client, planner_info: Planner) -> Result<Planner, MyError> {
        let _stmt = "insert into planner default values returning $table_fields;";
        let _stmt = _stmt.replace("$table_fields", &Planner::sql_table_fields());
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;
        
        client.query(
            &statement, 
            &[

            ]
        )
        .await
        .map_err(MyError::PGError)?
        .iter()
        .map(|row| Planner::from_row_ref(row).unwrap())
        .collect::<Vec<Planner>>()
        .pop()
        .ok_or(MyError::NotFound)       
    }

    pub async fn create_person(client: &Client, person_info: Person) -> Result<Person, MyError> {
        let _stmt = "insert into person(person_name) values($1) returning $table_fields;";
        let _stmt = _stmt.replace("$table_fields", &Person::sql_fields());
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.query(
            &statement,
            &[
                &person_info.person_name
            ]
        )
        .await
        .map_err(MyError::PGError)?
        .iter()
        .map(|row| Person::from_row_ref(row).unwrap())
        .collect::<Vec<Person>>()
        .pop()
        .ok_or(MyError::NotFound)
    }

    pub async fn modify_person(client: &Client, person_info: Person) -> Result<Person, MyError> {
        let _stmt = "update person set person_name = $1 where person_id = $2 returning $table_fields;";
        let _stmt = _stmt.replace("$table_fields", &Person::sql_table_fields());
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.query(
            &statement,
            &[
                &person_info.person_name,
                &person_info.person_id,
            ]
        )
        .await
        .map_err(MyError::PGError)?
        .iter()
        .map(|row| Person::from_row_ref(row).unwrap())
        .collect::<Vec<Person>>()
        .pop()
        .ok_or(MyError::NotFound)
    }

    pub async fn delete_person(client: &Client, person_info: Person) -> Result<u64, MyError> {
        let _stmt = "DELETE FROM person WHERE person_id = $1;";
        let _stmt = _stmt.to_string();
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.execute(
            &statement,
            &[
                &person_info.person_id.unwrap().to_string(),
            ]
        )
        .await
        .map_err(MyError::PGError)
    }

    pub async fn create_affiliation(client: &Client, affiliation_info: Affiliation) -> Result<Affiliation, MyError> {
        let _stmt = "insert into affiliation(person_id, organization_id) values ($1, $2) returning $table_fields;";
        let _stmt = _stmt.replace("$tale_fields", &Affiliation::sql_table_fields());
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.query(
            &statement,
            &[
                &affiliation_info.person_id,
                &affiliation_info.organization_id,
            ]
        )
        .await
        .map_err(MyError::PGError)?
        .iter()
        .map(|row| Affiliation::from_row_ref(row).unwrap())
        .collect::<Vec<Affiliation>>()
        .pop()
        .ok_or(MyError::NotFound)
    }

    pub async fn delete_affiliation(client: &Client, affiliation_info: Affiliation) -> Result<u64, MyError> {
        let _stmt = "delete from affiliation where affiliation_id = $1;";
        let _stmt = _stmt.to_string();
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.execute(
            &statement,
            &[&affiliation_info.affiliation_id.unwrap().to_string(),
            ]
        )
        .await
        .map_err(MyError::PGError)
    }

    pub async fn create_organization(client: &Client, organization_info: Organization) -> Result<Organization, MyError> {
        let _stmt = "insert into organization(organization_name) values ($1, $2) returing $table_fields;";
        let _stmt = _stmt.replace("$table_fields", &Organization::sql_table_fields());
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.query(
            &statement,
            &[
                &organization_info.organization_name,
            ]
        )
        .await
        .map_err(MyError::PGError)?
        .iter()
        .map(|row| Organization::from_row_ref(row).unwrap())
        .collect::<Vec<Organization>>()
        .pop()
        .ok_or(MyError::NotFound)
    }

    pub async fn delete_organization(client: &Client, organization_info: Organization) ->Result<u64, MyError> {
        let _stmt = "delete from organization where organization_id = $1;";
        let _stmt = _stmt.to_string();
        let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

        client.execute(
            &statement,
            &[
                &organization_info.organization_id.unwrap().to_string(),
            ]
        )
        .await
        .map_err(MyError::PGError)
    }


}

mod handlers {
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    use crate::{
        db, 
        errors::MyError, 
        models::{
            Event, 
            Person, 
            Affiliation, 
            Organization, Plan
        }
    };

    pub async fn create_event(
        event: web::Json<Event>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {

        let event_info: Event = event.into_inner();

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_event = db::create_event(&client, event_info).await?;

        Ok(HttpResponse::Ok().json(new_event))
    }

    pub async fn modify_event(
        event: web::Json<Event>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, MyError> {
        let event_info: Event = event.into_inner();

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let modified_event = db::modify_event(&client, event_info).await?;

        Ok(HttpResponse::Ok().json(modified_event))

    }

    pub async fn delete_event(
        event: web::Json<Event>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, MyError> {
        let event_info = event.into_inner();

        let client = db_pool.get().await.map_err(MyError::PoolError)?;

        let nb_deleted_event = db::delete_event(&client, event_info).await?;

        match nb_deleted_event {
            0 => Ok(HttpResponse::NotFound().finish()),
            _ => Ok(HttpResponse::Ok().finish()),
        }
    }

    pub async fn get_events(
        person: web::Json<Person>,
        db_pool: web::Data<Pool>
    ) -> Result<HttpResponse, MyError> {

        let person_info = person.into_inner();

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let events = db::get_events(&client, person_info).await?;

        Ok(HttpResponse::Ok().json(events))
    }

    pub async fn create_plan(
        plan: web::Json<Plan>,
        db_pool: web::Data<Pool>
    ) -> Result<HttpResponse, MyError> {
        let plan_info = plan.into_inner();

        let client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_plan = db::create_plan(&client, plan_info).await?;

        Ok(HttpResponse::Ok().json(new_plan))
    }

    pub async fn create_person(
        person: web::Json<Person>,
        db_pool: web::Data<Pool>
    ) -> Result<HttpResponse, MyError> {
        let person_info = person.into_inner();

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let person = db::create_person(&client, person_info).await?;

        Ok(HttpResponse::Ok().json(person))

    }

    pub async fn modify_person(
        person: web::Json<Person>,
        db_pool: web::Data<Pool>
    ) -> Result<HttpResponse, MyError> {
        let person_info = person.into_inner();

        let client = db_pool.get().await.map_err(MyError::PoolError)?;

        let person = db::modify_person(&client, person_info).await?;

        Ok(HttpResponse::Ok().json(person))
    }

    pub async fn delete_person(
        person: web::Json<Person>,
        db_pool: web::Data<Pool>
    ) -> Result<HttpResponse, MyError> {
        let person_info = person.into_inner();


        let client = db_pool.get().await.map_err(MyError::PoolError)?;

        let nb_deleted_person = db::delete_person(&client, person_info).await?;

        match nb_deleted_person {
            0 => Ok(HttpResponse::NotFound().finish()),
            1 => Ok(HttpResponse::Ok().finish()),
            _ => Ok(HttpResponse::InternalServerError().finish())
        }
    }

    pub async fn create_affiliation(
        affiliation: web::Json<Affiliation>,
        db_pool: web::Data<Pool>
    ) -> Result<HttpResponse, MyError> {
        let affiliation_info = affiliation.into_inner();

        let client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_affiliation = db::create_affiliation(&client, affiliation_info).await?;

        Ok(HttpResponse::Ok().json(new_affiliation))
    }

    pub async fn delete_affiliation(
        affiliation: web::Json<Affiliation>,
        db_pool: web::Data<Pool>
    ) -> Result<HttpResponse, MyError> {
        let affiliation_info = affiliation.into_inner();

        let client = db_pool.get().await.map_err(MyError::PoolError)?;

        let nb_delete_affiliation = db::delete_affiliation(&client, affiliation_info).await?;

        match nb_delete_affiliation {
            0 => Ok(HttpResponse::NotFound().finish()),
            1 => Ok(HttpResponse::Ok().finish()),
            _ => Ok(HttpResponse::InternalServerError().finish())
        }
    }

    pub async fn create_organization(
        organization: web::Json<Organization>,
        db_pool: web::Data<Pool>
    ) -> Result<HttpResponse, MyError> {
        let organization_info = organization.into_inner();

        let client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_organization = db::create_organization(&client, organization_info).await?;

        Ok(HttpResponse::Ok().json(new_organization))
    }

    pub async fn delete_organization(
        organization: web::Json<Organization>,
        db_pool: web::Data<Pool>
    ) -> Result<HttpResponse, MyError> {
        let organization_info = organization.into_inner();

        let client = db_pool.get().await.map_err(MyError::PoolError)?;

        let nb_delete_organization = db::delete_organization(&client, organization_info).await?;

        match nb_delete_organization {
            0 => Ok(HttpResponse::NotFound().finish()),
            1 => Ok(HttpResponse::Ok().finish()),
            _ => Ok(HttpResponse::InternalServerError().finish())
        }
    }
    

}



#[cfg(test)]
mod tests {
    use crate::models::{
        Person,
        Event,
        Organization
    };
    use crate::handlers::{
        create_person,
        delete_person,
        create_event,
        delete_event,
        create_organization,
        delete_organization,
    };

    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::test;

    #[actix_web::test]
    async fn test_create_delete_person() {
        let person = Person {
            person_id: None,
            person_name : "GDVCB".to_string(),
            planner_id: None
        };
        
        dotenv().ok();

        let config_ = Config::builder()
            .add_source(::config::Environment::default())
            .build()
            .unwrap();

        let config: ExampleConfig = config_.try_deserialize().unwrap();

        let pool = config.pg.create_pool(None, NoTls).unwrap();
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(web::resource("/users")
                    .route(web::post().to(create_person))
                    .route(web::delete().to(delete_person))
                )
        ).await;

        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(person)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        // let resp = create_person(req, pool).await?;
        // assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_delete_event() {
        let event = Event {
            event_id: None,
            event_name : "anniv GDVCB".to_string(),
            event_description: "Petit anniv".to_string(),
            event_location: "Paris".to_string(),
        };
        
        dotenv().ok();

        let config_ = Config::builder()
            .add_source(::config::Environment::default())
            .build()
            .unwrap();

        let config: ExampleConfig = config_.try_deserialize().unwrap();

        let pool = config.pg.create_pool(None, NoTls).unwrap();
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(web::resource("/events")
                    .route(web::post().to(create_event))
                )
        ).await;

        let req = test::TestRequest::post()
            .uri("/events")
            .set_json(event)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        // let resp = create_person(req, pool).await?;
        // assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_delete_organization() {
        let organization  = Organization {
            organization_id: None,
            organization_name : "festival_a".to_string(),
            planner_id: None,
        };
        
        dotenv().ok();

        let config_ = Config::builder()
            .add_source(::config::Environment::default())
            .build()
            .unwrap();

        let config: ExampleConfig = config_.try_deserialize().unwrap();

        let pool = config.pg.create_pool(None, NoTls).unwrap();
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(web::resource("/organizations")
                    .route(web::post().to(create_organization))
                )
        ).await;

        let req = test::TestRequest::post()
            .uri("/organizations")
            .set_json(organization)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
 
    }

    // #[actix_web::test]
    // async fn test_index_not_ok() {
        // let req = test::TestRequest::default().to_http_request();
        // let resp = index(req).await;
        // assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    // }
}

use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::{
    create_event, 
    modify_event, 
    get_events, 
    delete_event,
    create_plan, 
    create_person, 
    modify_person,
    delete_person,
    create_affiliation,
    delete_affiliation,
    create_organization,
    delete_organization,
};
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
            .service(web::resource("/events")
                .route(web::post().to(create_event))
                .route(web::patch().to(modify_event))
                .route(web::get().to(get_events))
                .route(web::delete().to(delete_event))
            )
            .service(web::resource("/plans")
                .route(web::post().to(create_plan))
            )
            .service(web::resource("/users")
                .route(web::post().to(create_person))
                .route(web::patch().to(modify_person))
                .route(web::delete().to(delete_person))
            )
            .service(web::resource("/affiliations")
                .route(web::post().to(create_affiliation))
                .route(web::delete().to(delete_affiliation))
            )
            .service(web::resource("/organizations")
                .route(web::post().to(create_organization))
                .route(web::delete().to(delete_organization))
            )   
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}