use anyhow::Error;
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Interface, Object, SimpleObject};

use crate::loaders::{PlayerLoader, TeamLoader};

#[derive(Clone, Interface)]
#[graphql(
    field(name = "name", type = "String"),
    field(name = "teams", type = "Vec<Team>")
)]
pub enum IServer {
    SquadServer(SquadServer),
}

#[derive(Clone, Default, SimpleObject)]
#[graphql(complex)]
pub struct SquadServer {
    pub name: String,
}

#[ComplexObject]
impl SquadServer {
    async fn teams<'a>(&self, ctx: &'a Context<'_>) -> Result<Vec<Team>, Error> {
        let loader = ctx.data_unchecked::<DataLoader<TeamLoader>>();
        let key = "my_key".to_string();
        let r = loader.load_one(key).await.unwrap().unwrap_or_default();
        Ok(r)
    }
}

#[derive(Clone, Default, SimpleObject)]
#[graphql(complex)]
pub struct Team {
    pub name: String,
}

#[ComplexObject]
impl Team {
    async fn players<'a>(&self, ctx: &'a Context<'_>) -> Result<Vec<String>, Error> {
        let loader = ctx.data_unchecked::<DataLoader<PlayerLoader>>();
        let key = "my_key".to_string();
        let r = loader.load_one(key).await.unwrap().unwrap_or_default();
        Ok(r)
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn servers(&self, _ctx: &Context<'_>) -> Result<Vec<IServer>, Error> {
        let mut list = vec![];

        for i in 0..2 {
            let name = format!("server-{}", i);
            list.push(IServer::SquadServer(SquadServer { name }));
        }

        Ok(list)
    }
}
