#[derive(Debug, Deserialize)]
struct ReleaseListPage {
    page: i32,
    pages: i32,
    releases: Vec<Release>,
    rows_per_page: i32,
    total: i32,
}
