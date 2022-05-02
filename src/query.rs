use anyhow::{anyhow, ensure, Context as Context3, Error};
use async_graphql::dataloader::DataLoader;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{
    ComplexObject, Context, EmptySubscription, Enum, InputObject, InputValueError,
    InputValueResult, Interface, Object, Scalar, ScalarType, Schema, SimpleObject, ID,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
// use async_graphql::{Context, Enum, Interface, Object, SimpleObject, ID};

#[derive(Clone, Interface)]
#[graphql(field(name = "teams", type = "Vec<String>"))]
pub enum IServer {
    SquadServer(SquadServer),
    // PsServer(PsServer),
}

#[derive(Clone, Default, SimpleObject)]
#[graphql(complex)]
pub struct SquadServer {
    pub id: ID,
}

// impl SquadServer {
//     fn consume_a2s_info(&mut self, v: query2::A2sInfo) {
//         self.map = v.map;
//         // self.version = v.version;
//         self.slots = v.slots;
//     }

// }

#[ComplexObject]
impl SquadServer {
    async fn teams<'a>(&self, _ctx: &'a Context<'_>) -> Vec<String> {
        vec!["Team1".to_string(), "Team2".to_string()]
    }
}

pub struct Query;

#[Object]
impl Query {
    // #[tracing::instrument(skip(self, ctx))]
    async fn server_by_name(&self, ctx: &Context<'_>, name: String) -> Result<IServer, Error> {
        // info!("get_server_by_name");

        let r = IServer::SquadServer(SquadServer {
            id: ID("123".to_string()),
        });
        Ok(r)
    }
}
