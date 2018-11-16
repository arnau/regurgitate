use crate::context::Context;
use crate::{Record, Records, Source, Storage};
use csv;
use std::path::PathBuf;
use std::fs::File;
use reqwest::{header, Client, StatusCode};
use std::error::Error;
use std::fmt;

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

    fn fetch(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let res = &mut self
            .client
            .get(&format!("{}{}", &self.context.origin(), path))
            .header(header::ACCEPT, header::HeaderValue::from_static("text/csv"))
            .send()?;

        match res.status() {
            StatusCode::OK => {
                let mut rdr = csv::Reader::from_reader(res);
                for result in rdr.deserialize() {
                    let mut record: Record = result?;
                    record.cast(&self.context.schema());
                    self.records.push(record);
                }

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
    fn write(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let checksum = self.records.checksum();
        let path = path.join(checksum).with_extension("csv");
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
