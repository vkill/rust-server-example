use crate::GraphqlSchema;
use repository::Repository;

pub struct State {
    pub repository: Repository,
    pub jwt_hs_secret: String,
    pub graphql_schema: GraphqlSchema,
}

impl State {
    pub fn new(
        repository: Repository,
        repository_for_graphql: Repository,
        jwt_hs_secret: String,
    ) -> Self {
        let graphql_schema = GraphqlSchema::new(repository_for_graphql);

        Self {
            repository,
            jwt_hs_secret,
            graphql_schema,
        }
    }
}
