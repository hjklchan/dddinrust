use uuid::Uuid;

pub struct Role {
    id: Uuid,

    name: String,
    slug: String,
    description: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum RoleError {
    #[error("name can not be empty")]
    EmptyName,
    #[error("slug can not be empty")]
    EmptySlug,
}

impl Role {
    pub fn new(name: String, slug: String, description: Option<String>) -> Result<Role, RoleError> {
        if name.is_empty() {
            return Err(RoleError::EmptyName);
        }
        if slug.is_empty() {
            return Err(RoleError::EmptySlug);
        }

        return Ok(Role {
            id: Uuid::new_v4(),
            name,
            slug,
            description,
        });
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn slug(&self) -> String {
        self.slug.clone()
    }
}
