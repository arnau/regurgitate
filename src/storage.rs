use crate::table::{Source, TableSchema};
use csv;
use reqwest::{self, header, Client, StatusCode};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Records(Vec<Record>);

impl Records {
    pub fn new() -> Records {
        Records(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn extend(&mut self, other: Records) {
        self.0.extend(other.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record(HashMap<String, String>);

impl Record {
    /// Retains any attribute found in the schema
    pub fn retain(&mut self, schema: &TableSchema) {
        let id = self
            .0
            .get("key")
            .expect("Missing 'key' attribute")
            .to_owned();
        self.0.insert("id".to_owned(), id);
        self.0.retain(|ref k, _| schema.contains_column(&k));
    }

    pub fn prune(&mut self, source_id: &str) {
        self.0.remove("index-entry-number");
        self.0.remove("entry-number");
        self.0.remove("entry-timestamp");
        self.0.remove(source_id);
    }
}

pub trait Storage {
    fn read(&mut self) -> Result<(), Box<dyn Error>>;
    fn write(&self) -> Result<(), Box<dyn Error>>;
    fn records(&self) -> &Records;
}

/// A remote storage.
pub struct Remote {
    client: Client,
    source: Source,
    schema: TableSchema,
    records: Records,
}

impl Remote {
    pub fn new(source: Source, schema: TableSchema) -> Remote {
        Remote {
            client: Client::new(),
            source: source,
            schema: schema,
            records: Records::new(),
        }
    }

    fn fetch(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let res = &mut self
            .client
            .get(&format!("{}{}", &self.source.url(), path))
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

impl Storage for Remote {
    fn read(&mut self) -> Result<(), Box<dyn Error>> {
        &self.fetch("/records");

        Ok(())
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn records(&self) -> &Records {
        &self.records
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
