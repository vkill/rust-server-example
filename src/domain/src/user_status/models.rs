use super::UserStatusError;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum UserStatus {
    Active = 1,
    Inactive = 2,
}

impl UserStatus {
    pub fn from_int(int: u32) -> Result<Self, UserStatusError> {
        Self::try_from(int).map_err(|e| e.into())
    }
}
