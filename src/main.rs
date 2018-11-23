extern crate csv;
extern crate regurgitate;
extern crate reqwest;
extern crate serde_json;
extern crate toml;

use regurgitate::context::Context;
use regurgitate::{Remote, Source, Storage};
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
// use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let catalogue = fs::read_dir("catalogue")?;

    for dir_entry in catalogue {
        let entry = dir_entry?;
        if entry.path().extension() == Some(OsStr::new("toml")) {
            println!("Processing {}", &entry.path().to_str().unwrap());
            process_dataset(&entry.path())?;
        }
    }

    Ok(())
}

fn process_dataset(context_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string(context_path)?;
    let context: Context = toml::from_str(&raw)?;

    let output_path = PathBuf::from(format!("data/{}/{}", &context.org_id(), &context.id()));
    let snapshots_path = output_path.join("snapshots");

    if !output_path.exists() {
        fs::create_dir(&output_path)?;
    }

    if !snapshots_path.exists() {
        fs::create_dir(&snapshots_path)?;
    }

    let mut storage = Remote::new(context);
    storage.read()?;

    storage.write(output_path)?;

    Ok(())
}
