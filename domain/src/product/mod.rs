use crate::shared::{identifier::Identifier, money::Money};

#[derive(Debug, Clone, Copy)]
pub struct Product<'a> {
    product_id: Identifier,

    name: &'a str,
    description: Option<&'a str>,
    unit_price: Money,
    uom: &'a str,
}

#[derive(Debug, thiserror::Error)]
pub enum ProductDomainError {
    #[error("product's name cannot be empty")]
    EmptyProructName,
    #[error("product's description cannot be empty")]
    EmptyProructDescription,
    #[error("negative price is not allowed")]
    NegativeUnitPrice,
    #[error("product's uom cannot be empty")]
    EmptyUom,
}

impl<'a> Product<'a> {
    fn new(
        product_id: Identifier,
        name: &'a str,
        description: Option<&'a str>,
        unit_price: Money,
        uom: &'a str,
    ) -> Self {
        Self {
            product_id,
            name,
            description,
            unit_price,
            uom,
        }
    }

    pub fn create(
        name: &'a str,
        description: Option<&'a str>,
        unit_price: Money,
        uom: &'a str,
    ) -> Result<Self, ProductDomainError> {
        if name.is_empty() {
            return Err(ProductDomainError::EmptyProructName);
        }
        if let Some(desc) = description {
            if desc.is_empty() {
                return Err(ProductDomainError::EmptyProructName);
            }
        }
        if uom.is_empty() {
            return Err(ProductDomainError::EmptyUom);
        }

        let product_id = Identifier::generate_v4();

        Ok(Self::new(product_id, name, description, unit_price, uom))
    }

    pub fn product_id(&self) -> Identifier {
        self.product_id
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn description(&self) -> Option<&'a str> {
        self.description
    }

    pub fn unit_price(&self) -> Money {
        self.unit_price
    }

    pub fn uom(&self) -> &'a str {
        self.uom
    }

    pub fn update_unit_price(&mut self, value: Money) {
        self.unit_price = value;
    }

    pub fn rename(&mut self, value: &'a str) -> Result<(), ProductDomainError> {
        if self.name().is_empty() {
            return Err(ProductDomainError::EmptyProructName);
        }

        self.name = value;

        Ok(())
    }

    pub fn update_description(&mut self, value: &'a str) -> Result<(), ProductDomainError> {
        if value.is_empty() {
            return Err(ProductDomainError::EmptyProructDescription);
        }

        self.description = Some(value);

        Ok(())
    }

    pub fn update_uom(&mut self, value: &'a str) -> Result<(), ProductDomainError> {
        if value.is_empty() {
            return Err(ProductDomainError::EmptyUom);
        }

        self.uom = value;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{product::Product, shared::money::Money};

    #[test]
    fn test_create_product() {
        let price = Money::try_from_cents(10000).unwrap();
        let product = Product::create("DDD in Rust", None, price, "本").unwrap();
        println!("{:#?}", product);
    }

    #[test]
    fn test_rename() {
        let price = Money::try_from_cents(10000).unwrap();
        let mut product = Product::create("DDD in Rust", None, price, "本").unwrap();
        println!("before rename: {:#?}", product);

        product.rename("Domain Driven Design in Rust").unwrap();
        println!("after rename: {:#?}", product);
    }

    #[test]
    fn test_update_unit_price() {
        let price = Money::try_from_cents(10000).unwrap();
        let mut product = Product::create("DDD in Rust", None, price, "本").unwrap();
        println!("before update: {:?}", product.unit_price());

        product.update_unit_price(Money::try_from_cents(999).unwrap());
        println!("after update: {:?}", product.unit_price());
    }
}
