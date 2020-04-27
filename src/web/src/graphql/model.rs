use async_graphql::{Context, FieldResult};
use repository::{domain, domain::UserRepository, Repository};

pub struct ContextUserID(pub i64);

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    #[field]
    async fn current_user_id<'a>(&self, ctx: &'a Context<'_>) -> Option<i64> {
        ctx.data_opt::<ContextUserID>().map(|c| c.0)
    }

    #[field]
    async fn me<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<User> {
        let user_id = ctx
            .data_opt::<ContextUserID>()
            .map(|c| c.0)
            .ok_or_else(|| "Unauthorized")?;

        let repository = ctx.data::<Repository>();

        let user = repository.get_user_by_id(user_id).await?;

        let user: User = user.into();

        Ok(user)
    }
}

pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[async_graphql::Object]
impl User {
    #[field]
    async fn id(&self) -> i64 {
        self.id
    }

    #[field]
    async fn username(&self) -> &str {
        &self.username
    }

    #[field]
    async fn email(&self) -> &str {
        &self.email
    }
}

impl From<domain::User> for User {
    fn from(u: domain::User) -> Self {
        Self {
            id: u.id,
            username: u.username,
            email: u.email,
        }
    }
}
