use std::{
    borrow::Cow,
    env::{self, VarError},
    num::ParseIntError,
    str::ParseBoolError,
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

impl Database {
    pub fn get_postgres_url(&self) -> String {
        format!(
            "postgres://{user}:{password}@{host}:{port}/{db}",
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            db = self.db,
        )
    }
}

pub struct MediaParserWorker {
    pub start_worker: bool,
}

pub struct Config {
    pub bot: Bot,
    pub database: Database,
    pub media_parser_worker: MediaParserWorker,
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("env error: {source} for key {key}")]
    Env {
        source: VarError,
        key: Cow<'static, str>,
    },
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    ParseBool(#[from] ParseBoolError),
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
        media_parser_worker: MediaParserWorker {
            start_worker: match env::var("START_MEDIA_PARSER_WORKER") {
                Ok(start_worker) => start_worker.parse()?,
                Err(err) => match err {
                    VarError::NotPresent => true,
                    VarError::NotUnicode(_) => {
                        return Err(ErrorKind::Env {
                            source: err,
                            key: "START_MEDIA_PARSER_WORKER".into(),
                        })
                    }
                },
            },
        },
    })
}
