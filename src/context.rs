
/// The local context describing what to expect from a data source.
///
/// The context schema is the base to derive the CSVW metadata file.
#[derive(Deserialize, Debug)]
pub struct Context {
    // TODO: Use complementary metadata
    origin: String,
    id: String,
    name: String,
    description: String,
    organisation: Organisation,
    schema: Schema,
}

impl Context {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn org_id(&self) -> &str {
        &self.organisation.id
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn schema_attributes(&self) -> &[Attribute] {
        self.schema.attributes()
    }
}


#[derive(Deserialize, Debug)]
pub struct Organisation {
    id: String
}


#[derive(Deserialize, Debug)]
pub struct Schema {
    primary_key: Vec<String>,
    attributes: Attributes,
}

impl Schema {
    pub fn attribute<'a>(&'a self, key: &'a str) -> Option<&'a Attribute> {
        self.attributes
            .iter()
            .find(|ref attr| attr.id == key)
    }

    pub fn attributes(&self) -> &[Attribute] {
        self.attributes.as_slice()
    }
}

type Attributes = Vec<Attribute>;

#[derive(Deserialize, Debug)]
pub struct Attribute {
    id: String,
    #[serde(default)]
    alias: Option<String>,
    #[serde(default)]
    description: Option<String>,
    datatype: String,
    #[serde(default)]
    format: Option<String>,
    #[serde(default)]
    required: bool,
    #[serde(default)]
    separator: Option<char>,
}

impl Attribute {
    pub fn id(&self) -> &str {
        if let Some(alias) = &self.alias {
            alias
        } else {
            &self.id
        }
    }

    pub fn alias(&self) -> &Option<String> {
        &self.alias
    }

    pub fn separator(&self) -> &Option<char> {
        &self.separator
    }

}
