use crate::shared::identifier::Identifier;

pub struct Product<'a> {
    product_id: Identifier,

    name: &'a str,
    description: Option<&'a str>,
    unit_price: f64,
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
        unit_price: f64,
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
        unit_price: f64,
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
        if unit_price < 0f64 {
            return Err(ProductDomainError::NegativeUnitPrice);
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

    pub fn unit_price(&self) -> f64 {
        self.unit_price
    }

    pub fn uom(&self) -> &'a str {
        self.uom
    }
}
