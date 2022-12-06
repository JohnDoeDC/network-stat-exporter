use clickhouse::Client;
use clickhouse::Row;
use serde::Serialize;
use std::sync::{Arc, Mutex};
pub struct Exporter{
    client : Arc<Mutex<Client>>
}

#[derive(Row, Serialize)]
pub struct StatEntity{

    /*
    CREATE TABLE network_stat
    (
        date DateTime DEFAULT now(),
        tx  UInt64,
        rx  UInt64,
        container_id  UInt32,
        user String,
        server_id UInt16,
        PRIMARY KEY container_id
    )
    ENGINE MergeTree()
    PARTITION BY toYYYYMM(date)
    ORDER BY (container_id, date)
    SETTINGS index_granularity=8192;
    
    */
    pub tx : u64, // tx :UInt64 
    pub rx: u64, //rx : UInt64 
    pub container_id : u32,
    pub user : String,
    pub server_id : u16

}

impl Exporter{
    pub fn new(host : impl Into<String>, user : impl Into<String>, password : impl Into<String>, database: impl Into<String>) -> Self{
        let client = Client::default()
        .with_url(host) //http://localhost:8123
        .with_user(user)
        .with_password(password)
        .with_database(database);
        Self { client : Arc::new(Mutex::new(client)) }
    }

    pub async fn insert(&self, tx: u64, rx: u64, container_id : u32, user : String, server_id: u16) -> Result<(), Box<dyn std::error::Error>>{
        match &self.client.clone().lock(){
            Ok(_transaction) => {
                let mut transaction = _transaction.insert("network_stat")?;
                transaction.write(&StatEntity { tx, rx, container_id, user, server_id }).await?;
                transaction.end().await?;
            }
            Err(e) => {
                return Err(e.to_string().into());
            }
        }
        Ok(())
    }

}