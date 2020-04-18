use super::UserPasswordError;

pub struct UserPassword {
    hash: String,
}

impl UserPassword {
    pub fn from_clear_text(clear_text_password: String) -> Result<Self, UserPasswordError> {
        let hash = bcrypt::hash(clear_text_password, bcrypt::DEFAULT_COST)?;
        Ok(Self { hash })
    }

    pub fn from_hash(hashed_password: String) -> Self {
        Self {
            hash: hashed_password,
        }
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn verify(&self, clear_text_password: &str) -> Result<bool, UserPasswordError> {
        Ok(bcrypt::verify(clear_text_password, &self.hash)?)
    }
}
