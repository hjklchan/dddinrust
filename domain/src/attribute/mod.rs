pub mod attribute_value;

use crate::{attribute::attribute_value::AttributeValue, shared::identifier::Identifier};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Attribute<'a> {
    attribute_id: Identifier,

    name: &'a str,
    code: &'a str,
    values: BTreeMap<&'a str, AttributeValue<'a>>,
}

#[derive(Debug, thiserror::Error)]
pub enum AttributeDomainError {
    #[error("attribute name cannot be empty")]
    EmptyAttributeName,
    #[error("attribute code cannot be empty")]
    EmptyAttributeCode,
    #[error("attribute value confict")]
    AttributeValueConflict,
    #[error("attribute value not found")]
    AttributeValueNotFound,
}

impl<'a> Attribute<'a> {
    fn new(
        attribute_id: Identifier,
        name: &'a str,
        code: &'a str,
        values: BTreeMap<&'a str, AttributeValue<'a>>,
    ) -> Self {
        Self {
            attribute_id,
            name,
            code,
            values,
        }
    }

    pub fn create(name: &'a str, code: &'a str) -> Self {
        Self::new(Identifier::generate_v4(), name, code, BTreeMap::new())
    }

    pub fn attribute_id(&self) -> Identifier {
        self.attribute_id
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn code(&self) -> &'a str {
        self.code
    }

    pub fn rename(&mut self, value: &'a str) -> Result<(), AttributeDomainError> {
        if value.is_empty() {
            return Err(AttributeDomainError::EmptyAttributeName);
        }

        self.name = value;

        Ok(())
    }

    pub fn change_code(&mut self, value: &'a str) -> Result<(), AttributeDomainError> {
        if value.is_empty() {
            return Err(AttributeDomainError::EmptyAttributeCode);
        }

        self.code = value;

        Ok(())
    }

    pub fn assign_value(&mut self, value: AttributeValue<'a>) -> Result<(), AttributeDomainError> {
        if self.values.contains_key(value.value()) {
            return Err(AttributeDomainError::AttributeValueConflict);
        }
        self.values.insert(value.code(), value);
        Ok(())
    }

    pub fn remove_value(&mut self, value_id: Identifier) {
        self.values
            .retain(|_k, v| v.attribute_value_id() != value_id);
    }

    pub fn contain_value(&self, value: AttributeValue) -> bool {
        self.values
            .values()
            .find(|rec| rec.attribute_value_id() == value.attribute_value_id())
            .is_some()
    }

    pub fn values(&self) -> Vec<&'a str> {
        self.values.values().map(|pre| pre.value()).collect()
    }

    pub fn codes(&self) -> Vec<&'a str> {
        self.values.values().map(|pre| pre.code()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::attribute::{Attribute, attribute_value::AttributeValue};

    #[test]
    fn test_attribute_assign_values() {
        let mut attr = Attribute::create("颜色", "color");
        attr.assign_value(AttributeValue::create("红色", "red"))
            .unwrap();
        attr.assign_value(AttributeValue::create("绿色", "green"))
            .unwrap();
        attr.assign_value(AttributeValue::create("黑色", "black"))
            .unwrap();

        println!("{:#?}", attr);
        println!("values: {:#?}", attr.values());
        println!("codes: {:#?}", attr.codes());
    }

    #[test]
    fn test_attribute_remove_value() {
        let mut attr = Attribute::create("颜色", "color");
        let value_red = AttributeValue::create("红色", "red");
        let value_green = AttributeValue::create("绿色", "green");
        let value_black = AttributeValue::create("黑色", "black");
        let value_white = AttributeValue::create("白色", "white");
        attr.assign_value(value_red).unwrap();
        attr.assign_value(value_green).unwrap();
        attr.assign_value(value_black).unwrap();
        attr.assign_value(value_white).unwrap();
        println!("before removed: {:#?}", attr);

        attr.remove_value(value_green.attribute_value_id());
        println!("before removed: {:#?}", attr);
    }
}
