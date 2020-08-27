#!/usr/bin/python3

import textwrap

RELEASES = [
    "F34",
    "F33",
    "F33C",
    "F33F",
    "F32",
    "F32C",
    "F32F",
    "F32M",
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
    "ELN",
]


def do_builds():
    import_template = "const JSON_{release}: &str = " + \
                      "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/builds_{release_lower}.json\");"

    test_template = textwrap.dedent(
        """\
        #[cfg(feature = "data-tests")]
        #[test]
        fn builds_dejson_{release_lower}() {{
            let builds: Vec<Build> = serde_json::from_str(&read_to_string(JSON_{release}).unwrap()).unwrap();

            for build in builds {{
                if !build.extra.is_empty() {{
                    println!("{{:#?}}", build.extra);
                }}
        
                assert!(build.extra.is_empty());
            }}
        }}
        """
    )

    contents = (
            "#![allow(unused_imports)]\n#![allow(dead_code)]\n\n" +
            "use std::fs::read_to_string;\n\nuse bodhi::Build;\n\n" +
            "\n".join(import_template.format(release=release, release_lower=release.lower()) for release in RELEASES) +
            "\n\n" +
            "\n".join(test_template.format(release=release, release_lower=release.lower()) for release in RELEASES)
    )

    with open("builds.rs", "w") as file:
        file.write(contents)


def do_comments():
    import_string = "const JSON: &str = " + \
                    "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/comments.json\");"

    test_string = textwrap.dedent(
        """\
        #[cfg(feature = "data-tests")]
        #[test]
        fn comments_dejson() {
            let comments: Vec<Comment> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();

            for comment in comments {
                if !comment.extra.is_empty() {
                    println!("{:#?}", comment.extra);
                }
        
                assert!(comment.extra.is_empty());
            }
        }
        """
    )

    contents = (
            "#![allow(unused_imports)]\n#![allow(dead_code)]\n\n" +
            "use std::fs::read_to_string;\n\nuse bodhi::Comment;\n\n" +
            import_string +
            "\n\n" +
            test_string
    )

    with open("comments.rs", "w") as file:
        file.write(contents)


def do_composes():
    import_string = "const JSON: &str = " + \
                    "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/composes.json\");"

    test_string = textwrap.dedent(
        """\
        #[cfg(feature = "data-tests")]
        #[test]
        fn composes_dejson() {
            let composes: Vec<Compose> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();

            for compose in composes {
                if !compose.extra.is_empty() {
                    println!("{:#?}", compose.extra);
                }

                assert!(compose.extra.is_empty());
            }
        }
        """
    )

    contents = (
            "#![allow(unused_imports)]\n#![allow(dead_code)]\n\n" +
            "use std::fs::read_to_string;\n\nuse bodhi::Compose;\n\n" +
            import_string +
            "\n\n" +
            test_string
    )

    with open("composes.rs", "w") as file:
        file.write(contents)


def do_overrides():
    import_template = "const JSON_{release}: &str = " + \
                      "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/overrides_{release_lower}.json\");"

    test_template = textwrap.dedent(
        """\
        #[cfg(feature = "data-tests")]
        #[test]
        fn overrides_dejson_{release_lower}() {{
            let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_{release}).unwrap()).unwrap();

            for o in os {{
                if !o.extra.is_empty() {{
                    println!("{{:#?}}", o.extra);
                }}
        
                assert!(o.extra.is_empty());
            }}
        }}
        """
    )

    contents = (
            "#![allow(unused_imports)]\n#![allow(dead_code)]\n\n" +
            "use std::fs::read_to_string;\n\nuse bodhi::Override;\n\n" +
            "\n".join(import_template.format(release=release, release_lower=release.lower()) for release in RELEASES) +
            "\n\n" +
            "\n".join(test_template.format(release=release, release_lower=release.lower()) for release in RELEASES)
    )

    with open("overrides.rs", "w") as file:
        file.write(contents)


def do_packages():
    import_string = "const JSON: &str = " + \
                    "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/packages.json\");"

    test_string = textwrap.dedent(
        """\
        #[cfg(feature = "data-tests")]
        #[test]
        fn packages_dejson() {
            let packages: Vec<Package> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();

            for package in packages {
                if !package.extra.is_empty() {
                    println!("{:#?}", package.extra);
                }
        
                assert!(package.extra.is_empty());
            }
        }
        """
    )

    contents = (
            "#![allow(unused_imports)]\n#![allow(dead_code)]\n\n" +
            "use std::fs::read_to_string;\n\nuse bodhi::Package;\n\n" +
            import_string +
            "\n\n" +
            test_string
    )

    with open("packages.rs", "w") as file:
        file.write(contents)


def do_releases():
    import_string = "const JSON: &str = " + \
                    "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/releases.json\");"

    test_string = textwrap.dedent(
        """\
        #[cfg(feature = "data-tests")]
        #[test]
        fn releases_dejson() {
            let releases: Vec<Release> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();

            for release in releases {
                if !release.extra.is_empty() {
                    println!("{:#?}", release.extra);
                }

                assert!(release.extra.is_empty());
            }
        }
        """
    )

    contents = (
            "#![allow(unused_imports)]\n#![allow(dead_code)]\n\n" +
            "use std::fs::read_to_string;\n\nuse bodhi::Release;\n\n" +
            import_string +
            "\n\n" +
            test_string
    )

    with open("releases.rs", "w") as file:
        file.write(contents)


def do_updates():
    import_template = "const JSON_{release}: &str = " + \
                      "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/updates_{release_lower}.json\");"

    test_template = textwrap.dedent(
        """\
        #[cfg(feature = "data-tests")]
        #[test]
        fn updates_dejson_{release_lower}() {{
            let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_{release}).unwrap()).unwrap();

            for update in updates {{
                if !update.extra.is_empty() {{
                    println!("{{:#?}}", update.extra);
                }}
        
                assert!(update.extra.is_empty());
            }}
        }}
        """
    )

    contents = (
            "#![allow(unused_imports)]\n#![allow(dead_code)]\n\n" +
            "use std::fs::read_to_string;\n\nuse bodhi::Update;\n\n" +
            "\n".join(import_template.format(release=release, release_lower=release.lower()) for release in RELEASES) +
            "\n\n" +
            "\n".join(test_template.format(release=release, release_lower=release.lower()) for release in RELEASES)
    )

    with open("updates.rs", "w") as file:
        file.write(contents)


def do_users():
    import_string = "const JSON: &str = " + \
                    "concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/users.json\");"

    test_string = textwrap.dedent(
        """\
        #[cfg(feature = "data-tests")]
        #[test]
        fn users_dejson() {
            let users: Vec<User> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();

            for user in users {
                if !user.extra.is_empty() {
                    println!("{:#?}", user.extra);
                }

                assert!(user.extra.is_empty());
            }
        }
        """
    )

    contents = (
            "#![allow(unused_imports)]\n#![allow(dead_code)]\n\n" +
            "use std::fs::read_to_string;\n\nuse bodhi::User;\n\n" +
            import_string +
            "\n\n" +
            test_string
    )

    with open("users.rs", "w") as file:
        file.write(contents)


def main() -> int:
    do_builds()
    do_comments()
    do_composes()
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

