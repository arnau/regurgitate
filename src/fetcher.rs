use csv;
use reqwest::header;
use reqwest::StatusCode;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

type Map = BTreeMap<String, Record>;

#[derive(Deserialize, Debug)]
pub struct Records(Map);

impl Records {
    pub fn new() -> Records {
        Records(Map::new())
    }

    pub fn get(&self, key: &str) -> Option<&Record> {
        self.0.get(key)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn fetch(url: &str, client: &reqwest::Client) -> Result<Records, Box<dyn Error>> {
        let records = Records::new();

        records.fetch_page(url, &client)
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn extend(&mut self, other: Records) {
        self.0.extend(other.0)
    }

    fn fetch_page(
        mut self,
        url: &str,
        client: &reqwest::Client,
    ) -> Result<Records, Box<dyn Error>> {
        let mut res = client
            .get(url)
            .header(
                header::ACCEPT,
                header::HeaderValue::from_static("application/json"),
            ).send()?;

        match res.status() {
            StatusCode::OK => {
                let records: Records = res.json()?;
                self.extend(records);

                if let Some(link_header) = res.headers().get(header::LINK) {
                    println!("Headers:\n{:#?}", &link_header);

                    if link_header.to_str()?.contains("next") {
                        return self.fetch_page(&format!("{}?page-index=2", url), &client);
                    }
                }

                Ok(self)
            }
            _ => Err(Box::new(FetchError)),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Record {
    #[serde(rename = "key")]
    id: String,
    #[serde(rename = "entry-number")]
    number: String,
    #[serde(rename = "entry-timestamp")]
    timestamp: String,
    #[serde(rename = "item")]
    blob: Vec<Blob>,
}

impl Record {
    pub fn blob(&self) -> &Blob {
        &self.blob[0]
    }
}

#[derive(Deserialize, Debug)]
pub struct Blob(BTreeMap<String, String>);

#[derive(Debug)]
pub struct FetchError;

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected error while fetching a records page")
    }
}

impl Error for FetchError {}
