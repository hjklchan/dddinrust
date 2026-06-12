use crate::{
    shared::{identifier::Identifier, money::Money},
    sku::sku_spec_assignment::SkuSpecificationAssignment,
};

#[derive(Debug)]
pub struct Sku<'u> {
    sku_id: Identifier,
    sku_code: &'u str,

    price: Money,
    stock: u64,

    specs: Vec<SkuSpecificationAssignment<'u>>,
}

#[derive(Debug)]
pub enum SkuError {
    ConflictAttribute,
    EmptySkuCode,
}

impl<'u> Sku<'u> {
    fn new(
        sku_id: Identifier,
        sku_code: &'u str,
        price: Money,
        stock: u64,
        specs: Vec<SkuSpecificationAssignment<'u>>,
    ) -> Self {
        Self {
            sku_id,
            sku_code,
            price,
            stock,
            specs,
        }
    }

    pub fn create(
        sku_code: &'u str,
        price: Money,
        stock: u64,
        specs: Vec<SkuSpecificationAssignment<'u>>,
    ) -> Result<Self, SkuError> {
        if sku_code.is_empty() {
            return Err(SkuError::EmptySkuCode);
        }

        Ok(Self::new(
            Identifier::generate_v4(),
            sku_code,
            price,
            stock,
            specs,
        ))
    }

    pub fn assign_specification(
        &mut self,
        assignment: SkuSpecificationAssignment<'u>,
    ) -> Result<(), SkuError> {
        // 不允许出现重复的规格
        if self
            .specs
            .iter()
            .any(|spec| spec.attribute_id() == assignment.attribute_id())
        {
            return Err(SkuError::ConflictAttribute);
        }

        self.specs.push(assignment);

        Ok(())
    }

    pub fn remove_specification(
        &mut self,
        target: SkuSpecificationAssignment<'u>,
    ) -> Result<(), SkuError> {
        self.specs
            .retain(|item| item.attribute_id() == target.attribute_id());

        Ok(())
    }

    pub fn attributes(&self) -> Vec<&'u str> {
        self.specs
            .iter()
            .map(|item| item.text_attribute())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        attribute::{Attribute, attribute_value::AttributeValue},
        shared::money::Money,
        sku::{sku::Sku, sku_spec_assignment::SkuSpecificationAssignment},
    };

    #[test]
    fn test_assign_specification() {
        // define attributes and values
        let mut color_attr = Attribute::create("Color", "color");
        let red = AttributeValue::create("Red", "color-red");
        let white = AttributeValue::create("White", "color-white");
        let green = AttributeValue::create("Green", "color-green");
        let blue = AttributeValue::create("Blue", "color-blue");
        color_attr.assign_value(red).unwrap();
        color_attr.assign_value(white).unwrap();
        color_attr.assign_value(green).unwrap();
        color_attr.assign_value(blue).unwrap();

        let mut sku = Sku::create(
            "TESTSKUCODE1000",
            Money::try_from_cents(999).unwrap(),
            1,
            Default::default(),
        )
        .unwrap();

        sku.assign_specification(SkuSpecificationAssignment::bind(
            color_attr.attribute_id(),
            green.attribute_value_id(),
            color_attr.name(),
            green.value(),
        ))
        .unwrap();

        println!("{:#?}", sku);
    }
}
