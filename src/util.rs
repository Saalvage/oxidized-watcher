use serenity::model::user::User;

pub fn display_name(user: &User) -> String {
    format!("{}#{}", user.name, user.discriminator)
}
