use clickhouse::Client;

#[derive(Clone)]
pub struct AnalyticsService {
    pub client: Client,
}

impl AnalyticsService {
    pub fn new(url: &str, database: &str, user: &str, password: &str) -> Self {
        let client = Client::default()
            .with_url(url)
            .with_database(database)
            .with_user(user)
            .with_password(password);
        Self { client }
    }
}
