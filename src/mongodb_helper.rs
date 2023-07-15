#[warn(dead_code)]
#[allow(unused_imports)]
use mongodb::{bson::doc, Client, options::{ClientOptions,FindOptions}};
use std::time;
use bson::{ Document};
use futures::{stream::TryStreamExt};
use serde_json::Value;
use crate::read_config::read_config;


pub async fn connect_to_mongodb(connection_string: &str) -> Result<Client,mongodb::error::Error > {
    let now = time::Instant::now();
    let client_options = ClientOptions::parse(connection_string).await?;
    let client = Client::with_options(client_options)?;
    println!("Connected to MongoDB SDL successfully in {:?}",now.elapsed());
    Ok(client)
}

pub async fn get_mongodb_data(database: String,collection: String,_filter: String) -> Result<Vec<String>,mongodb::error::Error> {
    let conf = read_config();
    let conf_json = conf["mongodb"].clone();

    let mdb_client = connect_to_mongodb(conf_json["connection_string"].as_str().unwrap()).await?;
    let mongodb_db = mdb_client.database(&database);
    let mongodb_collection = mongodb_db.collection::<Document>(&collection);

    // println!("{:?}",filter.as_str());
    // let doc_filter = doc! {"modbus_source":"conf1"};
    let doc_filter: Document = serde_json::from_str(_filter.as_str()).unwrap(); 
    // let now = time::Instant::now();
    // let projection_filter = bson::to_document(&{}).unwrap();
    // let find_options = FindOptions::builder().projection(projection_filter).build();
    // println!("{:?}",doc_filter);
    // println!("{:?}",find_options);
    let mut result = mongodb_collection.find(doc_filter,None).await?;
    // let mut result = mongodb_collection.find(filter, None).await?;
    let mut data_vec = Vec::new();
    while let Some(book) = result.try_next().await? {
        data_vec.push(book.to_string())
    }

    // let elapsed = now.elapsed();

    // log::info!("time elapsed to fetch data from mongodb for vessel {} staring {} till {} is {:?}",vessel_id,start_date,end_date,elapsed);

    Ok(data_vec)


}