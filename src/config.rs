use std::{
    borrow::Cow,
    env::{self, VarError},
    num::ParseIntError,
};

pub struct Bot {
    pub token: String,
}

pub struct Database {
    pub host: String,
    pub port: i16,
    pub user: String,
    pub password: String,
    pub db: String,
}

pub struct Config {
    pub bot: Bot,
    pub database: Database,
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("env error: {source} for key {key}")]
    Env {
        source: VarError,
        key: Cow<'static, str>,
    },
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub fn read_config_from_env() -> Result<Config, ErrorKind> {
    Ok(Config {
        bot: Bot {
            token: env::var("BOT_TOKEN").map_err(|err| ErrorKind::Env {
                source: err,
                key: "BOT_TOKEN".into(),
            })?,
        },
        database: Database {
            host: env::var("POSTGRES_HOST").map_err(|err| ErrorKind::Env {
                source: err,
                key: "POSTGRES_HOST".into(),
            })?,
            port: match env::var("POSTGRES_PORT") {
                Ok(port) => port.parse()?,
                Err(err) => match err {
                    VarError::NotPresent => 5432,
                    VarError::NotUnicode(_) => {
                        return Err(ErrorKind::Env {
                            source: err,
                            key: "POSTGRES_PORT".into(),
                        })
                    }
                },
            },
            user: match env::var("POSTGRES_USER") {
                Ok(user) => user,
                Err(err) => match err {
                    VarError::NotPresent => env::var("USER").map_err(|err| ErrorKind::Env {
                        source: err,
                        key: "POSTGRES_USER and USER".into(),
                    })?,
                    VarError::NotUnicode(_) => {
                        return Err(ErrorKind::Env {
                            source: err,
                            key: "POSTGRES_USER".into(),
                        })
                    }
                },
            },
            password: env::var("POSTGRES_PASSWORD").map_err(|err| ErrorKind::Env {
                source: err,
                key: "POSTGRES_PASSWORD".into(),
            })?,
            db: match env::var("POSTGRES_DB") {
                Ok(db) => db,
                Err(err) => match err {
                    VarError::NotPresent => env::var("USER").map_err(|err| ErrorKind::Env {
                        source: err,
                        key: "POSTGRES_DB and USER".into(),
                    })?,
                    VarError::NotUnicode(_) => {
                        return Err(ErrorKind::Env {
                            source: err,
                            key: "POSTGRES_DB".into(),
                        })
                    }
                },
            },
        },
    })
}
