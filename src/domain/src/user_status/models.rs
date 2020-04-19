use super::UserStatusError;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

#[derive(TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(i32)]
pub enum UserStatus {
    Active = 1,
    Inactive = 2,
}

impl UserStatus {
    pub fn from_int(int: i32) -> Result<Self, UserStatusError> {
        Self::try_from(int).map_err(|e| e.into())
    }
}
