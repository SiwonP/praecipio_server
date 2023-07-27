// use actix_web::http::StatusCode;
// use actix_web::{test, web, App};
// use tokio_postgres::NoTls;

// #[actix_web::test]
// async fn test_create_delete_person() {
//     let person = Person {
//         person_id: None,
//         person_name : "GDVCB".to_string(),
//         planner_id: None
//     };
    
//     dotenv().ok();

//     let config_ = Config::builder()
//         .add_source(::config::Environment::default())
//         .build()
//         .unwrap();

//     let config: ExampleConfig = config_.try_deserialize().unwrap();

//     let pool = config.pg.create_pool(None, NoTls).unwrap();
    
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .service(web::resource("/users")
//                 .route(web::post().to(create_person))
//                 .route(web::delete().to(delete_person))
//             )
//     ).await;

//     let req = test::TestRequest::post()
//         .uri("/users")
//         .set_json(person)
//         .to_request();

//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     let body: Person  = test::read_body_json(resp).await;
    
//     let req = test::TestRequest::delete()
//         .uri("/users")
//         .set_json(body)
//         .to_request();

//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

// }