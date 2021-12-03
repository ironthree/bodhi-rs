#!/usr/bin/python3

import jinja2

RELEASES = [
    "F36",
    "F36C",
    "F35",
    "F35C",
    "F35F",
    "F35M",
    "F34",
    "F34C",
    "F34F",
    "F34M",
    "F33",
    "F33C",
    "F33F",
    "F33M",
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
    "EPEL9",
    "EPEL9N",
    "EPEL8",
    "EPEL8M",
    "EPEL8N",
    "EPEL7",
    "EL6",
    "EL5",
    "ELN",
]


def do_builds():
    with open("templates/builds.jinja2") as file:
        template = jinja2.Template(file.read())

    with open("builds.rs", "w") as file:
        file.write(template.render(releases=RELEASES))


def do_overrides():
    with open("templates/overrides.jinja2") as file:
        template = jinja2.Template(file.read())

    with open("overrides.rs", "w") as file:
        file.write(template.render(releases=RELEASES))


def do_updates():
    with open("templates/updates.jinja2") as file:
        template = jinja2.Template(file.read())

    with open("updates.rs", "w") as file:
        file.write(template.render(releases=RELEASES))


def main() -> int:
    do_builds()
    do_overrides()
    do_updates()

    return 0


if __name__ == "__main__":
    try:
        exit(main())
    except KeyboardInterrupt:
        exit(0)

