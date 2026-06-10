use crate::shared::identifier::Identifier;

#[derive(Debug, Clone, Copy)]
pub struct AttributeValue<'a> {
    attribute_value_id: Identifier,

    value: &'a str,
    code: &'a str,
}

impl<'a> AttributeValue<'a> {
    fn new(attribute_value_id: Identifier, value: &'a str, code: &'a str) -> Self {
        Self {
            attribute_value_id,
            value,
            code,
        }
    }

    pub fn create(value: &'a str, code: &'a str) -> Self {
        Self::new(Identifier::generate_v4(), value, code)
    }

    pub fn attribute_value_id(&self) -> Identifier {
        self.attribute_value_id
    }

    pub fn value(&self) -> &'a str {
        self.value
    }

    pub fn code(&self) -> &'a str {
        self.code
    }

    pub fn change_value(&mut self, value: &'a str) -> Result<(), &'static str> {
        if value.is_empty() {
            return Err("value cannot be empty");
        }
        self.value = value;
        Ok(())
    }

    pub fn change_code(&mut self, value: &'a str) -> Result<(), &'static str> {
        if value.is_empty() {
            return Err("code cannot be empty");
        }
        self.code = value;
        Ok(())
    }
}
