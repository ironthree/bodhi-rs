#[derive(Debug, Deserialize)]
pub(crate) struct UserListPage {
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
    users: Vec<User>,
}
