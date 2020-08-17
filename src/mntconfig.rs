#[derive(Clone, serde::Deserialize)]
pub struct Config {
    pub boards: Vec<String>,
}
