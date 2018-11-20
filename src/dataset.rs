use blot::multihash::{Hash, Multihash, Sha3256};
use blot::tag::Tag;
use blot::Blot;
use crate::context::{Attribute, Schema, Context};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;

/// A data source should implement this trait
pub trait Source {
    fn read(&mut self) -> Result<(), Box<dyn Error>>;
    /// The context defining the dataset to be read.
    fn context(&self) -> &Context;
    /// The data checksum. None if there is no data.
    fn checksum(&self) -> Option<String>;
}

/// A data storage should implement this trait.
pub trait Storage: Source {
    fn write(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    pub fn push(&mut self, el: Record) {
        self.0.push(el)
    }

    pub fn sort(mut self) -> Self {
        self.0.sort_unstable_by(|a, b| a.id().cmp(&b.id()));
        self
    }

    pub fn as_slice(&self) -> &[Record] {
        self.0.as_slice()
    }

    pub fn checksum(&self) -> String {
        let digester = Sha3256;
        let mut list = self.0.clone();
        list.sort_unstable_by(|a, b| a.id().cmp(&b.id()));

        let digest = list
            .iter()
            .filter_map(|record| record.checksum())
            .collect::<Vec<String>>()
            .digest(digester);

        format!("{}", digest)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record(HashMap<String, String>);

impl Record {
    pub fn id(&self) -> Option<String> {
        self.0.get("_id").map(|s| s.clone())
    }

    pub fn checksum(&self) -> Option<String> {
        self.0.get("_checksum").map(|s| s.clone())
    }

    pub fn as_row(&self, attributes: &[Attribute]) -> Vec<String> {
        let mut result = Vec::new();

        for attr in attributes {
            if let Some(value) = self.0.get(attr.id()) {
                result.push(value.clone());
            } else {
                result.push("".to_string());
            }
        }

        result
    }

    /// Casts the record according to the given context schema.
    ///
    /// TODO: Shoud be part of the deserialization process.
    pub fn cast(&mut self, schema: &Schema) {
        let digester = Sha3256;
        let mut hash_list: Vec<Vec<u8>> = Vec::new();
        let mut result: HashMap<String, String> = HashMap::new();

        for (key, value) in &self.0 {
            if let Some(attr) = schema.attribute(&key) {
                if let Some(alias) = attr.alias() {
                    result.insert(alias.to_owned(), value.to_owned());
                } else {
                    result.insert(key.to_owned(), value.to_owned());
                }

                // Compute leaf hash
                if !value.is_empty() {
                    let mut res: Vec<u8> = Vec::with_capacity(64);
                    res.extend_from_slice(key.blot(&digester).as_ref());

                    if let Some(separator) = attr.separator() {
                        let set: HashSet<&str> = value.split(*separator).collect();
                        res.extend_from_slice(set.blot(&digester).as_ref());
                    } else {
                        res.extend_from_slice(value.blot(&digester).as_ref());
                    }

                    hash_list.push(res);
                }
            }
        }

        hash_list.sort_unstable();

        let digest = digester.digest_collection(Tag::Dict, hash_list);
        result.insert(
            "_checksum".to_owned(),
            format!("{}", Hash::new(digester, digest)),
        );

        self.0 = result;
    }
}
