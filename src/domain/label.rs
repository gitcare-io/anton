#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Label {
    id: u64,
    node_id: String,
    url: String,
    name: String,
    color: String,
    default: bool,
}
