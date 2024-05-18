use std::ops::Deref;

#[derive(Clone)]
pub struct TokenWrapper(pub String);

impl Deref for TokenWrapper {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
