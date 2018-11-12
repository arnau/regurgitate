//! Naive implementation of CSVW tabular data model
//!

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    #[serde(rename = "schema:name")]
    name: String,
    #[serde(rename = "schema:author")]
    author: Author,
    #[serde(rename = "schema:datePublished")]
    date_published: String,
    #[serde(rename = "schema:version")]
    version: String,
    #[serde(rename = "schema:sameAs")]
    redirect_url: String,
    url: String,
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
    #[serde(
        rename = "propertyUrl",
        skip_serializing_if = "Option::is_none"
    )]
    property_url: Option<String>,
    #[serde(rename = "valueUrl", skip_serializing_if = "Option::is_none")]
    value_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datatype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    #[serde(rename = "virtual", skip_serializing_if = "Option::is_none")]
    implicit: Option<bool>,
}

impl Column {
    pub fn separator(&self) -> &Option<char> {
        &self.separator
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableSchema {
    #[serde(rename = "aboutUrl")]
    about_url: String,
    columns: Vec<Column>,
    #[serde(rename = "primaryKey")]
    primary_key: String, // TODO: Could be an array too
}

impl TableSchema {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    #[serde(rename = "schema:name")]
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CopyrightHolder {
    #[serde(rename = "schema:name")]
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Source {
    url: String,
    #[serde(rename = "schema:identifier")]
    identifier: String,
}

impl Source {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableGroup {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "prov:wasDerivedFrom")]
    source: Source,
    #[serde(rename = "schema:name")]
    name: String,
    #[serde(rename = "schema:description")]
    description: String,
    #[serde(rename = "schema:author")]
    author: Author,
    #[serde(rename = "schema:copyrightHolder")]
    copyright_holder: CopyrightHolder,
    #[serde(rename = "schema:license")]
    license: String,
    #[serde(rename = "tableSchema")]
    table_schema: TableSchema,
    tables: Vec<Table>,
}

impl TableGroup {
    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn schema(&self) -> &TableSchema {
        &self.table_schema
    }
}
