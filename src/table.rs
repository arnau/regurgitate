//! Naive implementation of CSVW tabular data model
//!

use crate::context::{Attribute, Context, Organisation};
use crate::Source;

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "schema:identifier")]
    id: String,
    #[serde(rename = "schema:name")]
    name: String,
    #[serde(rename = "schema:description")]
    description: String,
    #[serde(rename = "schema:author")]
    author: Author,
    #[serde(rename = "schema:datePublished")]
    date_published: String,
    url: String,
    #[serde(rename = "schema:isBasedOn")]
    source: String,
    #[serde(rename = "schema:copyrightHolder")]
    copyright_holder: CopyrightHolder,
    #[serde(rename = "schema:license")]
    license: String,
    #[serde(rename = "tableSchema")]
    schema: Schema,
}

impl Table {
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

impl<T: Source> From<T> for Table {
    fn from(source: T) -> Table {
        let context = source.context();
        let checksum = source.checksum().unwrap_or("partial".into());

        Table {
            context: "http://www.w3.org/ns/csvw".into(),
            id: context.id().into(),
            name: context.name().into(),
            description: context.description().into(),
            author: context.owner().clone().into(),
            date_published: context.publication_date().into(),
            url: format!("/{}/{}/{}.csv", context.org_id(), context.id(), checksum),
            source: context.origin().into(),
            copyright_holder: CopyrightHolder {
                name: context.copyright_holder().name().into(),
            },
            license: context.license().into(),
            schema: context.clone().into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Column {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    titles: Option<String>, // TODO: Could be an array too
    #[serde(
        rename = "schema:description",
        skip_serializing_if = "Option::is_none"
    )]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datatype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(default)]
    required: bool,
}

impl Column {
    pub fn separator(&self) -> &Option<char> {
        &self.separator
    }
}

impl From<Attribute> for Column {
    fn from(attr: Attribute) -> Column {
        Column {
            name: Some(attr.id().into()),
            titles: Some(attr.id().into()),
            description: attr.description().clone(),
            datatype: Some(attr.datatype().into()),
            separator: attr.separator().clone(),
            format: attr.format().clone(),
            required: attr.required(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Schema {
    #[serde(rename = "aboutUrl")]
    about_url: String,
    columns: Vec<Column>,
    #[serde(rename = "primaryKey")]
    primary_key: String, // TODO: Could be an array too
}

impl Schema {
    pub fn contains_column(&self, column_id: &str) -> bool {
        self.columns
            .iter()
            .any(|ref col| col.name == Some(column_id.to_owned()))
    }

    pub fn column(&self, column_id: &str) -> Option<&Column> {
        self.columns
            .iter()
            .find(|ref col| col.name == Some(column_id.to_owned()))
    }
}

impl From<Context> for Schema {
    fn from(context: Context) -> Schema {
        let schema = context.schema();
        Schema {
            about_url: format!("{}/records/{{{}}}", context.origin(), schema.primary_key()),
            columns: schema
                .attributes()
                .into_iter()
                .map(|attr| Column::from(attr.clone()))
                .collect(),
            primary_key: schema.primary_key().into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    #[serde(rename = "schema:name")]
    name: String,
}

impl From<Organisation> for Author {
    fn from(org: Organisation) -> Author {
        Author {
            name: org.name().into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CopyrightHolder {
    #[serde(rename = "schema:name")]
    name: String,
}
