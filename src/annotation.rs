use crate::Url;

// {
//     "type": "Annotation",
//     "target": "countries.csv#cell=2,6-*,7",
//     "body": "These locations are of representative points.",
//     "motivation": "commenting"
// }
#[derive(Debug, Deserialize, Serialize)]
struct Annotation {
    target: String, // TODO: Should be Url but needs serde
    body: String,
    motivation: String, // commenting | editing
}
