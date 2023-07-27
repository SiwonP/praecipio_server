use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    db::errors::MyError, 
    db::models::{
        Event,
        Person,
        Plan, Planner,
        Affiliation,
        Organization
    }
};

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
            &event_info.event_id.unwrap(),
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
            &plan_info.plan_id.unwrap(),
        ]
    )
    .await
    .map_err(MyError::PGError)
}

pub async fn create_planner(client: &Client) -> Result<Planner, MyError> {
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

pub async fn delete_planner(client: &Client, planner_info: Planner) -> Result<u64, MyError> {
    let _stmt = "delete from planner where planner_id = $1;";
    let _stmt = _stmt.to_string();
    let statement = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

    client.execute(
        &statement,
        &[
            &planner_info.planner_id,
        ]
    )
    .await
    .map_err(MyError::PGError)
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
            &person_info.person_id.unwrap(),
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
        &[
            &affiliation_info.affiliation_id.unwrap(),
        ]
    )
    .await
    .map_err(MyError::PGError)
}

pub async fn create_organization(client: &Client, organization_info: Organization) -> Result<Organization, MyError> {
    let _stmt = "insert into organization(organization_name) values ($1) returning $table_fields;";
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
            &organization_info.organization_id.unwrap(),
        ]
    )
    .await
    .map_err(MyError::PGError)
}

