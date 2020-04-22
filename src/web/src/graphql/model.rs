use async_graphql::Context;

pub struct ContextUserID(pub Option<i64>);

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    #[field]
    async fn current_user_id<'a>(&self, ctx: &'a Context<'_>) -> Option<i64> {
        ctx.data::<ContextUserID>().0
    }
}
