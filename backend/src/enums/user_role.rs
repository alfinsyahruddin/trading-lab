use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum UserRole {
    Admin,
    Member,
}

impl UserRole {
    pub const fn is_admin(self) -> bool {
        matches!(self, Self::Admin)
    }
}

#[cfg(test)]
mod tests {
    use super::UserRole;

    #[test]
    fn deserialize_should_accept_uppercase_admin() {
        let role: UserRole = serde_json::from_str("\"ADMIN\"").expect("valid role JSON");
        assert_eq!(role, UserRole::Admin);
    }
}
