// Copied from traits section

A very common usage of `From` in Rust is with Error types. For example, we may have a large application that talks to
lots of different services, and we may want a single error handler at a lower point in the application. However, this
means we need to aggregate error types. `From` makes this easy:

```rust
// Code needed specifically for errors is hidden, use the eye icon to see it. ->

# use std::fmt;
# use std::error::Error;
#
# struct User;
#
# #[derive(Debug)]
# struct DbError;
# #[derive(Debug)]
# struct InvalidUserError;
# #[derive(Debug)]
# struct ProductStoreError;
#
#[derive(Debug)]
enum UserStoreError {
    DatabaseError(DbError),
    UnknownEmail(String),
}

# impl fmt::Display for UserStoreError {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
#         write!(f, "UserStoreError")
#     }
# }
#
# impl Error for UserStoreError {}
#
#[derive(Debug)]
enum ApplicationError {
    UserStoreError(UserStoreError),
    ProductStoreError(ProductStoreError),
}

# impl fmt::Display for ProductStoreError {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
#         write!(f, "ProductStoreError")
#     }
# }
#
# impl Error for ProductStoreError {}
#
impl From<UserStoreError> for ApplicationError {
    fn from(value: UserStoreError) -> ApplicationError {
        ApplicationError::UserStoreError(value)
    }
}

struct UserStore;

impl UserStore {
    fn get_user_by_email(email: &str) -> Result<User, UserStoreError> { 
        // ...
        Err(UserStoreError::UnknownEmail(String::from(email)))
    }
}

fn run_job() -> Result<(), ApplicationError> {
    let get_user_result = UserStore::get_user_by_email("no-a-real-email@example.com");
    let user = match get_user_result {
        Ok(user) => user,
        Err(error) => { return Err(error.into()); }
    };
    // ...
    Ok(())
}

fn main() {
    if let Err(error) = run_job() {
        eprintln!("{error:?}");
    }
}
```