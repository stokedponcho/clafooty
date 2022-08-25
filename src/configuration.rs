pub struct Configuration {
    pub token: String,
}

impl Configuration {
    pub fn new() -> Self {
        let token = env!("FOOTBALLDATA_API_AUTH_TOKEN");

        Configuration {
            token: token.to_string(),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}
