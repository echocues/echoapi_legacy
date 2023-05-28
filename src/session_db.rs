use std::collections::BTreeMap;
use rocket::{fairing::{Fairing, Info, Kind}, Rocket, Build};
use surrealdb::{dbs::Session, kvs::Datastore, sql::Value, Error, };

pub struct EchoDatabase {
    session: Session,
    datastore: Datastore,
}

impl EchoDatabase {
    pub async fn new(namespace: &str, database: &str, datastore: &str) -> Self {
        Self {
            session: Session::for_db(namespace.to_string(), database.to_string()),
            datastore: Datastore::new(datastore).await.unwrap(),
        }
    }

    pub async fn query(&self, statement: &str, vars: Option<BTreeMap<String, Value>>) -> Result<Vec<Value>, Error> {
        let responses = self.datastore.execute(statement, &self.session, vars, false).await?;
        let mut results = Vec::new();

        for response in responses {
            results.push(response.result?.first());
        }

        Ok(results)
    }
}

pub struct DbFairing;
#[rocket::async_trait]
impl Fairing for DbFairing {
    fn info(&self) -> Info {
        Info { name: "Surreal Database", kind: Kind::Ignite }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let database = EchoDatabase::new("echoserver", "echoserver", "memory").await;
        Ok(rocket.manage(database))
    }
}
