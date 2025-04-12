use crate::core::domain::entity::user::User;
use crate::shared::config::config::Config;
use crate::shared::worker::mailer::auth::error::AuthMailerError;
use crate::shared::worker::service::TaskContext;
use include_dir::{include_dir, Dir};
use serde_json::json;

// static WELCOME: Dir<'_> = include_dir!("src/shared/worker/mailer/auth/welcome");
// static FORGOT: Dir<'_> = include_dir!("src/shared/worker/mailer/auth/forgot");

// #[allow(clippy::module_name_repetitions)]
// pub struct AuthMailer {}
// impl Mailer for AuthMailer {}
// impl AuthMailer {
//     /// Sending WELCOME email the the given user
//     ///
//     /// # Errors
//     ///
//     /// When email sending is failed
//     pub async fn send_welcome(ctx: &TaskContext, user: &User) -> Result<(), AuthMailerError> {
//         todo!()
//     }
//
//     /// Sending FORGOT password email
//     ///
//     /// # Errors
//     ///
//     /// When email sending is failed
//     pub async fn forgot_password(ctx: &TaskContext, user: &User) -> Result<(), AuthMailerError> {
//         todo!()
//     }
// }
