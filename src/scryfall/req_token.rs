#[allow(dead_code)]
#[derive(Debug)]
pub enum ReqToken {
    Exact(String),
    Fuzzy(String),
    Set(String),
    Format(String),
    Face(String),
    Version(String),
    Pretty(bool)
}