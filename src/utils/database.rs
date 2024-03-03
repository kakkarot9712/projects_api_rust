use std::env;
extern crate dotenv;
use dotenv::dotenv;

use futures::TryStreamExt;
use mongodb::{Client, Database};

use crate::{languages::model::Languages, tools::model::Tools};

pub struct MongoClient {
    pub database: Database,
}

pub enum CollectionNames {
    LangList(Languages),
    TlsList(Tools),
}

impl MongoClient {
    pub async fn init() -> Self {
        println!("Connecting to DB...");
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v,
            Err(_) => panic!("Error Finding DB URL"),
        };
        let client = match Client::with_uri_str(uri).await {
            Ok(c) => c,
            Err(e) => {
                println!("{}", e);
                panic!("Something is wrong!");
            }
        };
        println!("Connection successfull... Getting Database");
        let database = client.database("portfolio");
        // println!("COL {:?}", database);
        MongoClient { database }
    }

    pub async fn get_all_languages(&self) -> Vec<Languages> {
        let col: Vec<Languages> = match self.database.collection("languages").find(None, None).await
        {
            Ok(d) => d.try_collect().await.unwrap_or_else(|_| vec![]),
            Err(_) => vec![],
        };
        col
    }

    pub async fn get_all_tools(&self) -> Vec<Tools> {
        let col: Vec<Tools> = match self.database.collection("tools").find(None, None).await {
            Ok(d) => d.try_collect().await.unwrap_or_else(|_| vec![]),
            Err(_) => vec![],
        };
        col
    }
}
