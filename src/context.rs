/// The local context describing what to expect from a data source.
///
/// The context schema is the base to derive the CSVW metadata file.
#[derive(Deserialize, Debug, Clone)]
pub struct Context {
    // TODO: Use complementary metadata
    origin: String,
    id: String,
    name: String,
    description: String,
    organisation: Organisation,
    owner: Organisation,
    copyright_holder: Organisation,
    schema: Schema,
    publication_date: String,
    license: String,
}

impl Context {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn organisation(&self) -> &Organisation {
        &self.organisation
    }

    pub fn org_id(&self) -> &str {
        &self.organisation.id
    }

    pub fn owner(&self) -> &Organisation {
        &self.owner
    }

    pub fn copyright_holder(&self) -> &Organisation {
        &self.copyright_holder
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

    pub fn publication_date(&self) -> &str {
        &self.publication_date
    }

    pub fn license(&self) -> &str {
        &self.license
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Organisation {
    id: String,
    name: String,
}

impl Organisation {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Schema {
    primary_key: Vec<String>,
    attributes: Attributes,
}

impl Schema {
    pub fn attribute<'a>(&'a self, key: &'a str) -> Option<&'a Attribute> {
        self.attributes.iter().find(|ref attr| attr.id == key)
    }

    pub fn attributes(&self) -> &[Attribute] {
        self.attributes.as_slice()
    }

    pub fn primary_key(&self) -> &str {
        &self.primary_key[0]
    }
}

type Attributes = Vec<Attribute>;

#[derive(Deserialize, Debug, Clone)]
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

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn datatype(&self) -> &str {
        &self.datatype
    }

    pub fn format(&self) -> &Option<String> {
        &self.format
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn separator(&self) -> &Option<char> {
        &self.separator
    }
}
