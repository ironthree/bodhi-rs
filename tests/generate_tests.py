#!/usr/bin/python3 -OO

import textwrap

# TODO: get releases from bodhi

RELEASES = [
    "F32",
    "F32C",
    "F31",
    "F31C",
    "F31F",
    "F31M",
    "F30",
    "F30C",
    "F30F",
    "F30M",
    "F29",
    "F29C",
    "F29F",
    "F29M",
    "F28",
    "F28C",
    "F28M",
    "F27",
    "F27M",
    "F26",
    "F25",
    "F24",
    "F23",
    "F22",
    "F21",
    "EPEL8",
    "EPEL8M",
    "EPEL7",
    "EL6",
    "EL5",
]


def do_builds():
    import_template = "const JSON_{release}: &str = " + \
                      "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/builds_{release_lower}.json\");"

    test_template = textwrap.dedent(
        """\
        #[test]
        fn builds_dejson_{release_lower}() {{
            let _: Vec<Build> = serde_json::from_str(&read_to_string(JSON_{release}).unwrap()).unwrap();
        }}
        """
    )

    contents = (
            "use std::fs::read_to_string;\nuse bodhi::data::Build;" +
            "\n\n" +
            "\n".join(import_template.format(release=release, release_lower=release.lower()) for release in RELEASES) +
            "\n\n" +
            "\n".join(test_template.format(release=release, release_lower=release.lower()) for release in RELEASES) +
            "\n"
    )

    with open("builds.rs", "w") as file:
        file.write(contents)


def do_overrides():
    import_template = "const JSON_{release}: &str = " + \
                      "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/overrides_{release_lower}.json\");"

    test_template = textwrap.dedent(
        """\
        #[test]
        fn overrides_dejson_{release_lower}() {{
            let _: Vec<Override> = serde_json::from_str(&read_to_string(JSON_{release}).unwrap()).unwrap();
        }}
        """
    )

    contents = (
            "use std::fs::read_to_string;\nuse bodhi::data::Override;" +
            "\n\n" +
            "\n".join(import_template.format(release=release, release_lower=release.lower()) for release in RELEASES) +
            "\n\n" +
            "\n".join(test_template.format(release=release, release_lower=release.lower()) for release in RELEASES) +
            "\n"
    )

    with open("overrides.rs", "w") as file:
        file.write(contents)


def do_packages():
    import_string = "const JSON: &str = " + \
                    "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/packages.json\");"

    test_string = textwrap.dedent(
        """\
        #[test]
        fn packages_dejson() {
            let _: Vec<Package> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();
        }
        """
    )

    contents = (
            "use std::fs::read_to_string;\nuse bodhi::data::Package;" +
            "\n\n" +
            import_string +
            "\n\n" +
            test_string +
            "\n"
    )

    with open("packages.rs", "w") as file:
        file.write(contents)


def do_releases():
    import_string = "const JSON: &str = " + \
                    "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/releases.json\");"

    test_string = textwrap.dedent(
        """\
        #[test]
        fn releases_dejson() {
            let _: Vec<Release> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();
        }
        """
    )

    contents = (
            "use std::fs::read_to_string;\nuse bodhi::data::Release;" +
            "\n\n" +
            import_string +
            "\n\n" +
            test_string +
            "\n"
    )

    with open("releases.rs", "w") as file:
        file.write(contents)


def do_updates():
    import_template = "const JSON_{release}: &str = " + \
                      "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/updates_{release_lower}.json\");"

    test_template = textwrap.dedent(
        """\
        #[test]
        fn updates_dejson_{release_lower}() {{
            let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_{release}).unwrap()).unwrap();
        }}
        """
    )

    contents = (
            "use std::fs::read_to_string;\nuse bodhi::data::Update;" +
            "\n\n" +
            "\n".join(import_template.format(release=release, release_lower=release.lower()) for release in RELEASES) +
            "\n\n" +
            "\n".join(test_template.format(release=release, release_lower=release.lower()) for release in RELEASES) +
            "\n"
    )

    with open("updates.rs", "w") as file:
        file.write(contents)


def do_users():
    import_string = "const JSON: &str = " + \
                    "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/users.json\");"

    test_string = textwrap.dedent(
        """\
        #[test]
        fn users_dejson() {
            let _: Vec<User> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();
        }
        """
    )

    contents = (
            "use std::fs::read_to_string;\nuse bodhi::data::User;" +
            "\n\n" +
            import_string +
            "\n\n" +
            test_string +
            "\n"
    )

    with open("users.rs", "w") as file:
        file.write(contents)


def main() -> int:
    do_builds()
    do_overrides()
    do_packages()
    do_releases()
    do_updates()
    do_users()

    return 0


if __name__ == "__main__":
    try:
        exit(main())
    except KeyboardInterrupt:
        exit(0)

