#[derive(Debug, Deserialize)]
struct StackListPage {
    page: i32,
    pages: i32,
    rows_per_page: i32,
    stacks: Vec<Stack>,
    total: i32,
}
