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
