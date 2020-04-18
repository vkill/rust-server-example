use repository::Repository;

pub struct State {
    pub repository: Repository,
    pub jwt_hs_secret: String,
}
