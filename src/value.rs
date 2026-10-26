use std::fmt;

pub struct Value(pub f64);
pub struct ValueArray(pub Vec<Value>);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
