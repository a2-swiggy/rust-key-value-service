use futures::{future, Future};
use actix::{Actor, Context, Message, Handler, AsyncContext};
use std::collections::HashMap;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use r2d2::Pool;
use std::time::Duration;
use crate::models::{Pair, NewPair};
use crate::schema::config::dsl::config;
use diesel::prelude::*;

pub type Result<T> = std::result::Result<T, ConfigStoreError>;

#[derive(Debug)]
pub enum Request {
    FetchPairs(Vec<String>),
    DeletePair(String),
    AddPair(String, String),
    UpdatePair(String, String),
    FetchAll()
}

#[derive(Debug)]
pub enum Response {
    Pairs(HashMap<String, String>),
    Ok(())
}

impl Message for Request {
    type Result = Result<Response>;
}

enum DeferredWork {
    UpdatePairs,
}

impl Message for DeferredWork {
    type Result = Result<()>;
}

pub struct ConfigStore {
    pool: Pool<ConnectionManager<MysqlConnection>>,
    pairs: HashMap<String, String>
}

impl ConfigStore {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Self {
        Self {
            pool,
            pairs: HashMap::with_capacity(1000)
        }
    }
}

impl Actor for ConfigStore {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(DeferredWork::UpdatePairs);
        ctx.run_interval(Duration::new(15, 0), move |_act, ctx| {
            ctx.notify(DeferredWork::UpdatePairs);
        });
    }
}

impl Handler<Request> for ConfigStore {
    type Result = Result<Response>;
    fn handle(&mut self, msg: Request, ctx: &mut Context<Self>) -> Result<Response> {
        match msg {
            Request::AddPair(name, value) => {
                self.pool.get()
                    .map_err(ConfigStoreError::from)
                    .and_then(|conn| {
                        diesel::insert_into(config)
                            .values(&NewPair { name: &name, value: &value })
                            .execute(&conn)
                            .expect("Error saving new post");
                        Ok(Response::Ok(()))
                    })
            }
            Request::FetchAll() => Ok(Response::Pairs(self.pairs.to_owned())),
            _ => Ok(Response::Ok(()))
        }
    }
}

impl Handler<DeferredWork> for ConfigStore {
    type Result = Result<()>;
    fn handle(&mut self, _msg: DeferredWork, _ctx: &mut Context<Self>) -> Result<()> {
        self.pool.get()
            .map_err(ConfigStoreError::from)
            .and_then(|conn| {
                let results = config.load::<Pair>(&conn).expect("Error loading key value pairs");
                let mut map: HashMap<String, String> = HashMap::with_capacity(results.len());
                for pair in results {
                    map.insert(pair.name, pair.value);
                }

                self.pairs = map;

                Ok(())
            })
    }
}

#[derive(Debug, Fail)]
pub enum ConfigStoreError {
    #[fail(display = "error: {}", err_str)]
    Custom {
        err_str: String,
    }
}

impl From<r2d2::Error> for ConfigStoreError {
    fn from(err: r2d2::Error) -> ConfigStoreError {
        ConfigStoreError::Custom {
            err_str: err.to_string()
        }
    }
}