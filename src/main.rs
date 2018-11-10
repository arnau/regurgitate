extern crate csv;
extern crate regurgitate;
extern crate reqwest;
extern crate serde_json;

use regurgitate::Records;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://country.register.gov.uk/records";
    let client = reqwest::Client::new();

    let records = Records::fetch(url, &client)?;

    println!("{:#?}", &records.len());
    println!("{:#?}", &records.get("PL"));

    Ok(())
}
