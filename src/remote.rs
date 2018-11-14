use crate::table::Schema;
use crate::{Record, Records, Source, Storage};
use csv;
use reqwest::{header, Client, StatusCode};
use std::error::Error;
use std::fmt;

/// A remote storage.
pub struct Remote {
    client: Client,
    source: String,
    schema: Schema,
    records: Records,
}

impl Remote {
    pub fn new(source: &str, schema: Schema) -> Remote {
        Remote {
            client: Client::new(),
            source: source.to_owned(),
            schema: schema,
            records: Records::new(),
        }
    }

    fn fetch(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let res = &mut self
            .client
            .get(&format!("{}{}", &self.source, path))
            .header(header::ACCEPT, header::HeaderValue::from_static("text/csv"))
            .send()?;

        match res.status() {
            StatusCode::OK => {
                let mut rdr = csv::Reader::from_reader(res);
                for result in rdr.deserialize() {
                    let mut record: Record = result?;
                    record.retain(&self.schema);
                    println!("{:?}", record);
                }
                // let records: Records = res.json()?;
                // self.extend(records);

                // if let Some(link_header) = res.headers().get(header::LINK) {
                //     println!("Headers:\n{:#?}", &link_header);

                //     if link_header.to_str()?.contains("next") {
                //         return self.fetch_page(&format!("{}?page-index=2", url), &client);
                //     }
                // }

                Ok(())
            }
            _ => Err(Box::new(FetchError)),
        }
    }
}

impl Source for Remote {
    fn read(&mut self) -> Result<(), Box<dyn Error>> {
        &self.fetch("/records");

        Ok(())
    }

    fn records(&self) -> &Records {
        &self.records
    }
}

impl Storage for Remote {
    fn write(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FetchError;

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected error while fetching a records page")
    }
}

impl Error for FetchError {}
