extern crate csv;
extern crate regurgitate;
extern crate reqwest;
extern crate serde_json;

use regurgitate::storage::{Remote, Storage};
use regurgitate::table::TableGroup;
use std::error::Error;
use std::fs;
// use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    // let url = "https://country.register.gov.uk/records";
    // let client = reqwest::Client::new();

    // let records = Records::fetch(url, &client)?;

    // println!("{:#?}", &records);
    // println!("{:#?}", &records.len());
    // // println!("{:#?}", &records.get("PL"));

    let filename = "./catalogue/country.json";
    let raw = fs::read_to_string(filename)?;
    let table_group: TableGroup = serde_json::from_str(&raw)?;
    // println!("{}", serde_json::to_string(&table_group)?);

    let mut storage = Remote::new(table_group.source().clone(), table_group.schema().clone());
    storage.read()?;

    println!("{:#?}", &storage.records());

    Ok(())
}
