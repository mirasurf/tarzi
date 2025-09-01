#[derive(Debug, Clone, PartialEq)]
pub enum AutoSwitchStrategy {
    Smart,
    None,
}

impl From<&str> for AutoSwitchStrategy {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "smart" => AutoSwitchStrategy::Smart,
            "none" => AutoSwitchStrategy::None,
            _ => AutoSwitchStrategy::Smart,
        }
    }
}
