use async_graphql::{
    Context, CustomValidator, EmptyMutation, EmptySubscription, InputObject, Object, Schema,
};

pub type AppSchema = Schema<AppQuery, EmptyMutation, EmptySubscription>;

//
// Model
//

struct PasswordValidator;

impl CustomValidator<String> for PasswordValidator {
    /// Check if `value` only contains allowed chars
    fn check(&self, value: &String) -> Result<(), String> {
        let allowed_chars = ['1', '2', '3', '4', '5'];

        if value
            .chars()
            .all(|c| allowed_chars.contains(&c.to_ascii_lowercase()))
        {
            Ok(())
        } else {
            Err(format!("illegal char in password: `{}`", value))
        }
    }
}

#[derive(InputObject)]
pub struct ChangePasswordInput {
    #[graphql(validator(min_length = 3, max_length = 63))]
    pub namespace: String,
    //
    #[graphql(validator(min_length = 6, max_length = 16))]
    #[graphql(validator(custom = "PasswordValidator"))]
    pub new_password: String,
}

//
// Query
//

#[derive(Default)]
pub struct AppQuery;

#[Object]
impl AppQuery {
    pub async fn change_password(&self, _ctx: &Context<'_>, _input: ChangePasswordInput) -> String {
        "password changed".to_string()
    }
}
