use crate::domain::user::entities::User as UserEntity;

use std::sync::Arc;
use telers::{
    client::Bot,
    context::Context,
    errors::ExtractionError,
    extractors::{from_context, FromEventAndContext},
    types::Update,
};

from_context!([Client], UserEntity, "db_user");
