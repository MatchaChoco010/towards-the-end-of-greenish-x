use serde::Deserialize;

#[derive(Deserialize)]
pub enum OpeningData {
    Random { data: Vec<(f64, OpeningData)> },
    Sequence { data: Vec<OpeningData> },
    Data { data: String },
}
