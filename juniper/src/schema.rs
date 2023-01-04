use chrono::{DateTime, Utc};
use juniper::{FieldResult};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use juniper::{EmptySubscription, RootNode};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
    create_date: String,
    create_date1: DateTime<Utc>,
    create_date2: i32,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
    create_date: String,
    create_date1: DateTime<Utc>,
    create_date2: i32,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
            create_date: Utc::now().timestamp().to_string(),
            create_date1: Utc::now(),
            create_date2: Utc::now().timestamp() as i32,
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
            create_date: Utc::now().timestamp().to_string(),
            create_date1: Utc::now(),
            create_date2: Utc::now().timestamp() as i32,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
