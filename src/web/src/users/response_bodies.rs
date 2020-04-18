use repository::domain;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UserResponseBody {
    user: User,
}

#[derive(Serialize, Debug)]
struct User {
    username: String,
    email: String,
    token: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
}

type UserToken = String;

impl From<(domain::User, UserToken)> for UserResponseBody {
    fn from(t: (domain::User, UserToken)) -> Self {
        let (user, token) = t;
        Self {
            user: User {
                username: user.username,
                email: user.email,
                token,
                first_name: user.profile.first_name,
                last_name: user.profile.last_name,
                phone: user.profile.phone,
            },
        }
    }
}
