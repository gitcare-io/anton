#[allow(dead_code)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Installation {
    id: u64,
    node_id: String,
}
