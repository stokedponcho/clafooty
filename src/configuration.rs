use std::env;

pub struct Configuration {
    pub token: String,
}

const AUTH_TOKEN_ENV_VARIABLE: &str = "API_DATAFOOTBALL_AUTH_TOKEN";

impl Configuration {
    pub fn new() -> Self {
        let token = env::var(AUTH_TOKEN_ENV_VARIABLE).expect(
            format!(
                "Environment variable {} could not be loaded",
                AUTH_TOKEN_ENV_VARIABLE
            )
            .as_str(),
        );

        Configuration { token }
    }
}
