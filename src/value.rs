use std::fmt;

pub struct Value(pub f64);
pub struct ValueArray(pub Vec<Value>);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn displays_numeric_value() {
        let value = Value(1.2);

        assert_eq!(value.to_string(), "1.2");
    }
}
