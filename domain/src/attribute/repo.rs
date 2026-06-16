use std::process::Output;

use crate::{attribute::Attribute, shared::identifier::Identifier};

pub enum AttributeRepositoryError {}

pub trait AttributeSyncRepository<'a> {
    fn find_by_id(
        &self,
        id: Identifier,
    ) -> impl Future<Output = Result<Attribute<'a>, AttributeRepositoryError>>;

    fn all(&self) -> impl Future<Output = Result<Vec<Attribute<'a>>, AttributeRepositoryError>>;

    fn create(
        &self,
        attribute: Attribute,
    ) -> impl Future<Output = Result<Attribute<'a>, AttributeRepositoryError>>;

    fn update(
        &self,
        attribute: Attribute,
    ) -> impl Future<Output = Result<(), AttributeRepositoryError>>;

    fn delete_by_id(
        &self,
        id: Identifier,
    ) -> impl Future<Output = Result<(), AttributeRepositoryError>>;
}

pub trait AttributeRepository<'a> {
    fn find_by_id(&self, id: Identifier) -> Result<Attribute<'a>, AttributeRepositoryError>;
    fn all(&self) -> Result<Vec<Attribute<'a>>, AttributeRepositoryError>;
    fn create(&self, attribute: Attribute) -> Result<Attribute<'a>, AttributeRepositoryError>;
    fn update(&self, attribute: Attribute) -> Result<(), AttributeRepositoryError>;
    fn delete_by_id(&self, id: Identifier) -> Result<(), AttributeRepositoryError>;
}
