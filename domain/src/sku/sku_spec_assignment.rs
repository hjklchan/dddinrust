use crate::shared::identifier::Identifier;

#[derive(Debug)]
pub struct SkuSpecificationAssignment<'a> {
    attribute_id: Identifier,
    attribute_value_id: Identifier,

    text_attribute: &'a str,
    text_value: &'a str,
}

impl<'a> SkuSpecificationAssignment<'a> {
    fn new(
        attribute_id: Identifier,
        attribute_value_id: Identifier,
        text_attribute: &'a str,
        text_value: &'a str,
    ) -> Self {
        Self {
            attribute_id,
            attribute_value_id,
            text_attribute,
            text_value,
        }
    }

    pub fn bind(
        attribute_id: Identifier,
        attribute_value_id: Identifier,
        text_attribute: &'a str,
        text_value: &'a str,
    ) -> Self {
        Self::new(attribute_id, attribute_value_id, text_attribute, text_value)
    }

    pub fn attribute_id(&self) -> Identifier {
        self.attribute_id
    }

    pub fn attribute_value_id(&self) -> Identifier {
        self.attribute_value_id
    }

    pub fn text_attribute(&self) -> &'a str {
        self.text_attribute
    }

    pub fn text_value(&self) -> &'a str {
        self.text_value
    }
}
