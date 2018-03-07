use utils::FullTextSpan;
use utils::reporting::structures::*;

pub struct Register {
    errors: Vec<Error>,
    warnings: Vec<Warning>,
    info: Vec<Info>,
}

impl Register {
    pub fn new() -> Register {
        Register {
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }

    pub fn cast_error(&mut self, full_span: FullTextSpan, description: String) {
        let err = Error { full_span: Some(full_span), description };
        self.errors.push(err)
    }

    pub fn cast_error_non_text(&mut self, description: String) {
        let err = Error { full_span: None, description };
        self.errors.push(err)
    }

    pub fn errors(&mut self) -> &mut Vec<Error> {
        &mut self.errors
    }

    pub fn cast_info(&mut self, full_span: FullTextSpan, description: String) {
        let info_detail = Info { full_span: Some(full_span), description };
        self.info.push(info_detail)
    }

    pub fn cast_info_non_text(&mut self, description: String) {
        let info_detail = Info { full_span: None, description };
        self.info.push(info_detail)
    }

    pub fn info(&mut self) -> &mut Vec<Info> {
        &mut self.info
    }

    pub fn cast_warning(&mut self, full_span: FullTextSpan, description: String) {
        let warning = Warning { full_span: Some(full_span), description };
        self.warnings.push(warning)
    }

    pub fn cast_warning_non_text(&mut self, description: String) {
        let warning = Warning { full_span: None, description };
        self.warnings.push(warning)
    }

    pub fn warnings(&mut self) -> &mut Vec<Warning> {
        &mut self.warnings
    }
}
