use std::env;

pub struct Environment {
    pub db_url: String,
    pub addr: String,
}

impl Environment {
    pub fn load() -> Self {
        let env_db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL mus in der .env gesetzt sein!");
        let env_ip_addr: String = env::var("IP_ADDR").expect("IP_ADDR mus in der .env gesetzt sein!");
        let env_port: String = env::var("PORT").expect("PORT mus in der .env gesetzt sein!");

        Self { db_url: env_db_url, addr: format!("{}:{}", env_ip_addr, env_port) }
    }
}