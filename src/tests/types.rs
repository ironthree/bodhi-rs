use crate::{Bug, TestCase};

#[test]
fn bug_url() {
    let bug = Bug {
        bug_id: 1234567,
        parent: false,
        security: false,
        title: None,
        extra: Default::default(),
    };

    assert_eq!(
        bug.url().to_string(),
        "https://bugzilla.redhat.com/show_bug.cgi?id=1234567"
    );
}

#[test]
fn testcase_url() {
    let testcase = TestCase {
        name: String::from("QA:Foo Bar Baz"),
        package: None,
        package_id: 0,
        extra: Default::default(),
    };

    assert_eq!(
        testcase.url().to_string(),
        "https://fedoraproject.org/wiki/QA:Foo_Bar_Baz"
    );
}
