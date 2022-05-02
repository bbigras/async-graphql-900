use std::{collections::HashMap, sync::Arc};

use anyhow::Error;
use async_graphql::{async_trait, dataloader::Loader};

use crate::query::Team;

pub struct TeamLoader {}

#[async_trait::async_trait]
impl Loader<String> for TeamLoader {
    type Value = Vec<Team>;
    type Error = Arc<Error>;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        println!("load");

        let mut map: HashMap<String, Self::Value> = HashMap::new();

        for key in keys.iter() {
            for i in 0..3 {
                let name = format!("team{}", i);

                let value = Team { name };

                if let Some(x) = map.get_mut(key) {
                    x.push(value);
                } else {
                    map.insert(key.to_string(), vec![value]);
                }
            }
        }

        Ok(map)
    }
}

pub struct PlayerLoader {}

#[async_trait::async_trait]
impl Loader<String> for PlayerLoader {
    type Value = Vec<String>;
    type Error = Arc<Error>;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let mut map: HashMap<String, Self::Value> = HashMap::new();

        for key in keys.iter() {
            for i in 0..3 {
                let value = format!("team{}", i);

                if let Some(x) = map.get_mut(key) {
                    x.push(value);
                } else {
                    map.insert(key.to_string(), vec![value]);
                }
            }
        }

        Ok(map)
    }
}
