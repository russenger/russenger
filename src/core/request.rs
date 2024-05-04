use crate::query::Query;
use crate::response_models::data::Data;

/// The `Req` struct represents a request from a user.
///
/// It contains the following fields:
/// * `user`: A `String` that represents the user who made the request.
/// * `query`: A `Query` that represents the query made by the user.
/// * `data`: A `Data` that represents the data associated with the request.
/// * `host`: A `String` that represents the host from which the request was made.
#[derive(Clone)]
pub struct Req {
    pub user: String,

    /// The `Query` struct represents a database query.
    ///
    /// This struct is used to interact with the database. It contains a `db` field, which is an instance of the `DB` enum that represents the database connection.
    ///
    /// # Fields
    ///
    /// * `db`: The database connection. This is an instance of the `DB` enum.
    ///
    /// # Methods
    ///
    /// * `new`: This method creates a new `Query`. It establishes a connection to the database and returns a `Query` with the established connection.
    /// * `migrate`: This method creates a new table `russenger_user` in the database. It returns a boolean indicating whether the operation was successful.
    /// * `create`: This method inserts a new user into the `russenger_user` table. It takes a user ID as an argument and returns a boolean indicating whether the operation was successful.
    /// * `set_action`: This method updates the action of a user in the `russenger_user` table. It takes a user ID and an action as arguments and returns a boolean indicating whether the operation was successful.
    ///
    /// # Examples
    ///
    /// Creating a new `Query` and using it to insert a new user into the database:
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// create_action!(Main, |res: Res, req: Req| async move {
    ///     req.set_action(&req.user, NextAction).await; // goto NextAction
    ///
    ///});
    /// create_action!(NextAction, |res: Res, req: Req| async move {});
    /// ```
    pub query: Query,
    pub data: Data,
    pub host: String,
}

impl Req {
    /// Creates a new `Req`.
    ///
    /// # Arguments
    ///
    /// * `user`: A string slice that holds the user.
    /// * `query`: A `Query` that holds the query.
    /// * `data`: A `Data` that holds the data.
    /// * `host`: A string slice that holds the host.
    ///
    /// # Returns
    ///
    /// A `Req` that contains the provided user, query, data, and host.
    pub fn new(user: &str, query: Query, data: Data, host: &str) -> Self {
        Self {
            user: user.to_owned(),
            query,
            data,
            host: host.to_owned(),
        }
    }
}
