use uuid::Uuid;

/// Represents a registered user in the system.
///
/// A user can create threads, write posts, and be assigned moderation roles.
///
/// # Examples
///
/// ```
/// use uuid::Uuid;
/// use coralbb_core::models::User;
///
/// let user = User {
///     id: Uuid::new_v4(),
///     username: String::from("kevin"),
///     email: String::from("kevin@example.com"),
/// };
/// ```
pub struct User {
    id: Uuid,
    name: String,
    email: String,
    password_hash: String,
}
