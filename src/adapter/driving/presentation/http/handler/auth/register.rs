use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::core::domain::valueobject::date::Timestamp;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserRegisterRequest {
    #[serde(default)]
    #[validate(length(
        min = 3,
        max = 20,
        message = "First name is not valid. It should be between 3 and 20 characters."
    ))]
    pub name: String,

    #[serde(default)]
    #[validate(length(
        min = 3,
        max = 20,
        message = "Last name is not valid. It should be between 3 and 20 characters."
    ))]
    pub surname: String,

    #[serde(default)]
    #[validate(email(message = "Email address is not valid."))]
    pub email: String,

    #[serde(default)]
    #[validate(length(
        min = 8,
        message = "Password is not valid. It should be at least 8 characters."
    ))]
    pub password: String,
}

#[derive(Serialize, Debug)]
struct UserRegisterResponse {
    message: String,
    user: UserResponse,
}

#[derive(Serialize, Debug)]
struct UserResponse {
    pub user_id: Uuid,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub role: String,
    pub password: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

// impl From<RegisterError<ValidationErrors>> for ApiResponseData<ResponseError> {
// fn from(value: RegisterError<ValidationErrors>) -> Self {
//     match value {
//         RegisterError::BadClientData(err) => ApiResponseData::error(
//             Some(ResponseError::from(err)),
//             "invalid data from client",
//             StatusCode::BAD_REQUEST,
//         ),
//         RegisterError::UserAlreadyRegistered => {
//             ApiResponseData::error(None, "user already registered", StatusCode::FORBIDDEN)
//         }
//         RegisterError::DbInternalError
//         | RegisterError::HashingError
//         | RegisterError::InternalError => {
//             ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR)
//         }
//     }
// }
// }

// pub async fn register_handler<S>(
//     State(app): State<Arc<AppState<S>>>,
//     register_user: Json<UserRegisterRequest>,
// ) -> ApiResponse<UserRegisterResponse, ResponseError>
// where
//     S: UserManagement,
// {
//     register_user
//         .validate()
//         .map_err(RegisterError::BadClientData)?;
//
//     let result = app.user_service.register(&register_user).await;
//     match result {
//         Ok(registered_user) => {
//             let user_res = UserResponse {
//                 user_id: registered_user.id.unwrap(),
//                 name: registered_user.name,
//                 surname: registered_user.surname,
//                 email: registered_user.email,
//                 role: registered_user.role.as_string(),
//                 password: register_user.password.clone(),
//                 created_at: registered_user.created_at,
//                 updated_at: registered_user.updated_at,
//             };
//
//             let res = UserRegisterResponse {
//                 message: "User registered successfully! Please verify your email.".to_string(),
//                 user: user_res,
//             };
//
//             Ok(ApiResponseData::success_with_data(res, StatusCode::OK))
//         }
//         Err(error) => Err(ApiResponseData::from(error)),
//     }
// }
