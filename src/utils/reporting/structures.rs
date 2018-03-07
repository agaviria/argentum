use utils::FullTextSpan;

pub struct Error {
    pub full_span: Option<FullTextSpan>,
    pub description: String
}

pub struct Info {
    pub full_span: Option<FullTextSpan>,
    pub description: String
}

pub struct Warning {
    pub full_span: Option<FullTextSpan>,
    pub description: String
}
