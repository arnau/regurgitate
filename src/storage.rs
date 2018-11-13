use blot::multihash::{Hash, Multihash, Sha3256};
use blot::tag::Tag;
use blot::Blot;
use crate::table::{Schema, Source};
use csv;
use reqwest::{header, Client, StatusCode};
use std::collections::{HashMap, HashSet};
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
    pub fn retain(&mut self, schema: &Schema) {
        let id = self
            .0
            .get("key")
            .expect("Missing 'key' attribute")
            .to_owned();
        self.0.insert("id".to_owned(), id);

        self.0.retain(|ref k, _| schema.contains_column(&k));

        let checksum = self.checksum(&schema);
        self.0.insert("checksum".to_owned(), checksum);
    }

    /// Checksum is an implementation of a Blot dictionary after filtering out
    /// empty cells and tranforming multivalue cells into sets of values.
    pub fn checksum(&self, schema: &Schema) -> String {
        let digester = Sha3256;
        let mut list: Vec<Vec<u8>> = self
            .0
            .iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, v)| {
                let mut res: Vec<u8> = Vec::with_capacity(64);
                res.extend_from_slice(k.blot(&digester).as_ref());

                let col = schema.column(k).expect("Missing column");

                if let Some(separator) = col.separator() {
                    let set: HashSet<&str> = v.split(*separator).collect();
                    res.extend_from_slice(set.blot(&digester).as_ref());
                } else {
                    res.extend_from_slice(v.blot(&digester).as_ref());
                }

                res
            }).collect();

        list.sort_unstable();

        let digest = digester.digest_collection(Tag::Dict, list);
        let hash = Hash::new(digester, digest);

        format!("{}", hash)
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
    schema: Schema,
    records: Records,
}

impl Remote {
    pub fn new(source: Source, schema: Schema) -> Remote {
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
