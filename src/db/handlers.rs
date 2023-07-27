use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{
    db::db, 
    db::errors::MyError, 
    db::models::{
        Event, 
        Plan,
        Planner,
        Organization,
        Affiliation,
        Person
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
        1 => Ok(HttpResponse::Ok().finish()),
        _ => Ok(HttpResponse::InternalServerError().finish()),
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

pub async fn delete_plan(
    plan: web::Json<Plan>,
    db_pool: web::Data<Pool>
) -> Result<HttpResponse, MyError> {
    let plan_info = plan.into_inner();

    let client = db_pool.get().await.map_err(MyError::PoolError)?;

    let nb_delete_plan = db::delete_plan(&client, plan_info).await?;

    match nb_delete_plan {
        0 => Ok(HttpResponse::NotFound().finish()),
        1 => Ok(HttpResponse::Ok().finish()),
        _ => Ok(HttpResponse::InternalServerError().finish())
    }
}

pub async fn create_planner(
    db_pool: web::Data<Pool>
) -> Result<HttpResponse, MyError> {

    let client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_planner = db::create_planner(&client).await?;

    Ok(HttpResponse::Ok().json(new_planner))
}

pub async fn delete_planner(
    planner: web::Json<Planner>,
    db_pool: web::Data<Pool>
) -> Result<HttpResponse, MyError> {
    let planner_info = planner.into_inner();

    let client = db_pool.get().await.map_err(MyError::PoolError)?;

    let nb_delete_planner = db::delete_planner(&client, planner_info).await?;

    match nb_delete_planner {
        0 => Ok(HttpResponse::NotFound().finish()),
        1 => Ok(HttpResponse::Ok().finish()),
        _ => Ok(HttpResponse::InternalServerError().finish())
    }
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