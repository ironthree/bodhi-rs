#[derive(Debug, Deserialize)]
pub(crate) struct PackageListPage {
    packages: Vec<Package>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}
