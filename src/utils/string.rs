pub trait WhitespaceChecker {
    fn is_whitespace(&self) -> bool;
}

impl WhitespaceChecker for String {
    fn is_whitespace(&self) -> bool {
        self.trim().is_empty()
    }
}

impl WhitespaceChecker for str {
    fn is_whitespace(&self) -> bool {
        self.trim().is_empty()
    }
}

impl<S: WhitespaceChecker> WhitespaceChecker for Option<S> {
    fn is_whitespace(&self) -> bool {
        self.as_ref().map_or(false, S::is_whitespace)
    }
}
