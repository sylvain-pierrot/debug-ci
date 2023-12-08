#[derive(Debug, Clone)]
pub struct AppEnvironment {
    /// The address to listen on
    pub address: String,

    /// The port to listen on
    pub port: u16,

    /// The sqlite database url to use
    pub database_url: String,

    /// The username to use for basic auth
    pub admin_username: String,

    /// The password to use for basic auth
    pub admin_password: String,
}
