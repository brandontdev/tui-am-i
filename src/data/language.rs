use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Language {
    pub id: i64,
    pub name: String,
    pub description: String
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:#?}{}{}", self.id, self.name, self.description)
    }
}

impl Language {
    pub fn new() -> Self {
        Self::default()
    }
}