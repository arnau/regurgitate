use crate::context::Context;
use crate::link_header::Link;
use crate::url::Url;
use crate::{Record, Records, Source, Storage};
use csv;
use reqwest::{header, Client};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;

/// A remote storage.
pub struct Remote {
    client: Client,
    context: Context,
    records: Records,
}

impl Remote {
    pub fn new(context: Context) -> Remote {
        Remote {
            client: Client::new(),
            context: context,
            records: Records::new(),
        }
    }

    fn fetch(&mut self, url: &Url) -> Result<(), Box<dyn Error>> {
        println!("{}", url);
        let res = &mut self
            .client
            .get(url.as_str())
            .header(header::ACCEPT, header::HeaderValue::from_static("text/csv"))
            .send()?;

        if res.status().is_success() {
            let mut buf: Vec<u8> = vec![];
            res.copy_to(&mut buf)?;

            let mut rdr = csv::Reader::from_reader(buf.as_slice());
            for result in rdr.deserialize() {
                let mut record: Record = result?;
                record.cast(&self.context.schema());
                self.records.push(record);
            }

            if let Some(links) = res.headers().get(header::LINK).map(|link_header| {
                Link::from_header(link_header).expect("Link to be of the right format")
            }) {
                if let Some(next_url) = links.iter().find(|lnk| lnk.is_next()) {
                    return self.fetch(&url.join(next_url.to_str())?);
                }
            }

            Ok(())
        } else {
            Err(Box::new(FetchError))
        }
    }

    fn write_data(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let mut wtr = csv::Writer::from_writer(file);

        let attrs = &self.context.schema_attributes();

        wtr.serialize(attrs.iter().map(|attr| attr.id()).collect::<Vec<&str>>())?;

        for record in self.records.as_slice() {
            let row = record.as_row(&self.context.schema_attributes());
            wtr.serialize(row)?;
        }

        wtr.flush()?;

        Ok(())
    }

    fn write_metadata(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn records(&self) -> &Records {
        &self.records
    }
}

impl Source for Remote {
    fn read(&mut self) -> Result<(), Box<dyn Error>> {
        let url = Url::parse(self.context.origin())?.join("/records")?;
        &self.fetch(&url);

        Ok(())
    }
}

impl Storage for Remote {
    fn write(&mut self, base_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let checksum = self.records.checksum();
        let snapshot_path = base_path.join("snapshots");
        let path = snapshot_path.join(checksum).with_extension("csv");
        self.write_data(&path)?;
        self.write_metadata(&path.with_extension("csv-metadata.json"))?;

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
