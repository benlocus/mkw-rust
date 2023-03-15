/// GameSpyDatabase.rs
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response, Session};

type DB = (Datastore, Session);

struct GameSpyDatabase {
    datastore: Datastore,
    session: Session,
}

impl GameSpyDatabase {
    async fn new() -> Self {
        Self {
            datastore: Datastore::new("memory")
                .await
                .expect("Unable to start database in memory."),
            session: Session::for_db("namespace", "database"),
        }
    }

    async fn intitialize(&self) -> () {
        self.datastore
            .execute(
                include_str!("./sql/initialize.sql"),
                &self.session,
                None,
                false,
            )
            .await
            .expect("Failed to run initialization SQL.");
    }

    async fn get_next_free_profileid(&self) -> Result<i32> {
        let sql = "SELECT math::max(profileid) as next_available_profileid FROM users GROUP BY profileid;";
        let responses = self
            .datastore
            .execute(sql, &self.session, None, false)
            .await?;

        let id: Option<i32> = GameSpyDatabase::into_iter_object(responses)?
            .next()
            .transpose()?
            .and_then(|obj| {
                obj.get("next_available_profileid")
                    .map(|id| id.to_number().as_int() as i32)
            });

        let next_available: i32 = match id {
            Some(id) => id + 1,
            None => 1,
        };

        Ok(next_available)
    }

    async fn check_user_exists(&self, userid: &str, gsbrcd: &str) -> Result<bool> {
        let sql = "SELECT count() as count FROM users WHERE userid = $userid AND gsbrcd = $gsbrcd";
        let vars: BTreeMap<String, Value> = [
            ("userid".into(), userid.into()),
            ("gsbrcd".into(), gsbrcd.into()),
        ]
        .into();

        let responses = self
            .datastore
            .execute(sql, &self.session, Some(vars), false)
            .await?;

        let response = GameSpyDatabase::into_iter_object(responses)?
            .next()
            .transpose()?
            .and_then(|obj| obj.get("count").map(|count| count.to_number().as_int()));

        match response {
            None => return Ok(false),
            Some(0) => return Ok(false),
            Some(_) => return Ok(true),
        };
    }

    async fn check_user_enabled(&self, userid: &str, gsbrcd: &str) -> Result<bool> {
        let sql = "SELECT enabled FROM users WHERE userid = $userid AND gsbrcd = $gsbrcd";
        let vars: BTreeMap<String, Value> = [
            ("userid".into(), userid.into()),
            ("gsbrcd".into(), gsbrcd.into()),
        ]
        .into();

        let responses = self
            .datastore
            .execute(sql, &self.session, Some(vars), false)
            .await?;

        let response = GameSpyDatabase::into_iter_object(responses)?
            .next()
            .transpose()?;

        let enabled = match response {
            None => return Ok(false),
            Some(obj) => obj
        };

        match enabled.get("enabled") {
            None => return Ok(false),
            Some(bool) => return match bool  {
                Value::True => Ok(true),
                _ => Ok(false)
            }
        };
    }

    fn into_iter_object(responses: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
        let result = responses
            .into_iter()
            .next()
            .map(|res| res.result)
            .transpose()?;

        match result {
            Some(Value::Array(arr)) => {
                let iter = arr.into_iter().map(|val| match val {
                    Value::Object(object) => Ok(object),
                    _ => Err(anyhow!("Record was not an object.")),
                });
                Ok(iter)
            }
            _ => Err(anyhow!("No records found.")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let database: GameSpyDatabase = GameSpyDatabase::new().await;
    database.intitialize().await;

    let datastore = &database.datastore;

    // Basic Create Statement
    let sql: &str = "CREATE users CONTENT {
    firstname: 'Ben',
    lastname: 'Duke',
    email: 'ben@tenaxstrategies.com',
    profileid: 1,
    zipcode: '02120'
};

CREATE users CONTENT {
    firstname: 'Jack',
    lastname: 'Waiss',
    email: 'jack@tenaxstrategies.com',
    profileid: 2,
    zipcode: '02120'
};";
    let responses: Vec<Response> = datastore
        .execute(sql, &database.session, None, false)
        .await?;

    let next_id = database.get_next_free_profileid().await?;

    println!("{next_id}");

    Ok(())
}

// async fn create_task(database: &GameSpyDatabase, title: &str, priority: i32) -> Result<String> {
//     let datastore = &database.datastore;
//
//     let sql: &str = "CREATE task CONTENT $data";
//
//     let data: BTreeMap<String, Value> = [
//         ("title".into(), title.into()),
//         ("priority".into(), priority.into()),
//     ]
//     .into();
//
//     let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();
//
//     let responses = datastore
//         .execute(sql, &database.session, Some(vars), false)
//         .await?;
//
//     into_iter_object(responses)?
//         .next()
//         .transpose()?
//         .and_then(|obj| obj.get("id").map(|id| id.to_string()))
//         .ok_or_else(|| anyhow!("No id returned."))
// }
// // Using a function with SurrealDB variables to execute sql --> returns id as string
// let task_1 = create_task(&database, "task #2", 5i32).await?;
// println!("{task_1}");
//
// // Basic Update Statement
// let sql: &str = "UPDATE $thing MERGE $data RETURN id";
// let data: BTreeMap<String, Value> = [
//     ("title".into(), "Task1 updated".into()),
//     ("done".into(), true.into()),
// ]
// .into();
// let vars: BTreeMap<String, Value> = [
//     ("thing".into(), thing(&task_1)?.into()),
//     ("data".into(), data.into()),
// ]
// .into();
// datastore
//     .execute(sql, &database.session, Some(vars), true)
//     .await?;
//
// // Basic Select Statement
// let sql: &str = "SELECT * FROM task";
// let responses: Vec<Response> = datastore
//     .execute(sql, &database.session, None, false)
//     .await?;
//
// for object in into_iter_object(responses)? {
//     println!("Record: {}", object?);
// }
