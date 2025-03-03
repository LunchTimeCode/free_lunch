use dotenvy::dotenv;
use libsql::{Builder, Database};
use rocket::tokio::sync::Mutex;
use std::{env, sync::Arc};

type LockedState = Arc<Mutex<State>>;

pub struct _State {
    state: LockedState,
}

impl _State {
    pub async fn get(&self) -> rocket::tokio::sync::MutexGuard<'_, State> {
        self.state.lock().await
    }

    pub async fn from_config(config: Config) -> Self {
        _State {
            state: Arc::new(Mutex::new(State::from_config(config).await)),
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct State {
    db: Database,
}

impl State {
    pub async fn from_config(config: Config) -> Self {
        let db = if config.turso_auth_token.is_none() {
            Builder::new_local("local.db").build().await.unwrap()
        } else {
            let url = config.turso_database_url;
            let token = config.turso_auth_token.unwrap();
            Builder::new_remote(url, token).build().await.unwrap()
        };
        let first_connection = db.connect();
        match first_connection {
            Ok(_) => {
                println!("Connected to database");
            }
            Err(err) => {
                eprintln!("Could not connect to db: {}", err);
            }
        }

        Self { db }
    }
}

pub async fn initial_state(config: Config) -> _State {
    _State::from_config(config).await
}

#[allow(unused)]
pub struct Config {
    turso_database_url: String,
    turso_auth_token: Option<String>,
    port: u16,
    address: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().expect(".env file not found");
        let turso_database_url: String =
            env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL not found!");
        let turso_auth_token: Option<String> = env::var("TURSO_AUTH_TOKEN").ok();

        Self {
            turso_database_url,
            turso_auth_token,
            port: 9999,
            address: "0.0.0.0".to_string(),
        }
    }
}
