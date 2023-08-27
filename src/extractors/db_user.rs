use crate::domain::user::entities::User as UserEntity;

use std::sync::Arc;
use telers::{
    client::Bot,
    context::Context,
    errors::ExtractionError,
    extractors::{from_context_impl, FromEventAndContext},
    types::Update,
};

from_context_impl!([Client], UserEntity, "db_user");
