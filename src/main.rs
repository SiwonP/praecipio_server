pub mod db;

#[cfg(test)]
mod tests {
    use crate::db::models::{
        Person,
        Event,
        Organization
    };
    use crate::db::handlers::{
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

        // Create the request that inserts a new person in the table
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(person)
            .to_request();

        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp.response().body());
        assert_eq!(resp.status(), StatusCode::OK);

        // let body: Person  = test::read_body_json(resp).await;
        
        // // Create the request that delete the previsouly inserted person in the database
        // let req = test::TestRequest::delete()
        //     .uri("/users")
        //     .set_json(body)
        //     .to_request();

        // let resp = test::call_service(&app, req).await;
        // assert_eq!(resp.status(), StatusCode::OK);

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
                    .route(web::delete().to(delete_event))
                )
        ).await;

        // Create the request that inserts a new event in the table
        let req = test::TestRequest::post()
            .uri("/events")
            .set_json(event)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Event  = test::read_body_json(resp).await;
        
        // Create the request that delete the previsouly inserted person in the database
        let req = test::TestRequest::delete()
            .uri("/events")
            .set_json(body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
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
                    .route(web::delete().to(delete_organization))
                )
        ).await;

        // Create the request that inserts a new organization in the table
        let req = test::TestRequest::post()
            .uri("/organizations")
            .set_json(organization)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Organization  = test::read_body_json(resp).await;
        
        // Create the request that delete the previsouly inserted organization in the database
        let req = test::TestRequest::delete()
            .uri("/organizations")
            .set_json(body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
 
    }


}

use ::config::Config;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use db::handlers::{
    create_event, 
    modify_event, 
    get_events, 
    delete_event,
    create_plan,
    delete_plan,
    create_person, 
    modify_person,
    delete_person,
    create_affiliation,
    delete_affiliation,
    create_organization,
    delete_organization,
    create_planner,
    delete_planner,
};
use tokio_postgres::NoTls;

use crate::db::config::ExampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();


    
        // .allowed_origin("https://www.rust-lang.org")
        // .allowed_methods(vec!["GET", "POST"])
        // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        // .allowed_header(header::CONTENT_TYPE)
        // .max_age(3600);


    let server = HttpServer::new(move || {

        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(web::resource("/events")
                .route(web::post().to(create_event))
                .route(web::patch().to(modify_event))
                .route(web::get().to(get_events))
                .route(web::delete().to(delete_event))
            )
            .service(web::resource("/plans")
                .route(web::post().to(create_plan))
                .route(web::delete().to(delete_plan))
            )
            .service(web::resource("/users")
                // .route(web::get().to(get_persons))
                .route(web::post().to(create_person))
                .route(web::patch().to(modify_person))
                .route(web::delete().to(delete_person))
            )
            .service(web::resource("/users/{person_id}/events")
                .route(web::get().to(get_events))
            )
            .service(web::resource("/planner")
                .route(web::post().to(create_planner))
                .route(web::delete().to(delete_planner))
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