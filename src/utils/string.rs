pub trait StringWithWhitespaceChecker {
    fn is_whitespace(&self) -> bool;
}

impl StringWithWhitespaceChecker for String {
    fn is_whitespace(&self) -> bool {
        self.trim().is_empty()
    }
}

impl StringWithWhitespaceChecker for str {
    fn is_whitespace(&self) -> bool {
        self.trim().is_empty()
    }
}



impl StringWithWhitespaceChecker for Option<String> {
    fn is_whitespace(&self) -> bool {
        self.as_ref().map_or(false, |s| s.trim().is_empty() )
    }
}