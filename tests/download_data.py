#!/usr/bin/python3

import argparse
import json
import os
import threading
import time
from typing import List

import requests

API_URL = "https://bodhi.fedoraproject.org"

ALL_RELEASES = [
    "F35",
    "F35C",
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
    "EPEL-8",
    "EPEL-8M",
    "EPEL-8N",
    "EPEL-7",
    "EL-6",
    "EL-5",
    "ELN",
]

ACTIVE_RELEASES = [
    "F35",
    "F35C",
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
    "EPEL-8",
    "EPEL-8M",
    "EPEL-8N",
    "EPEL-7",
    "EL-6",
    "ELN",
]

# change this to "ALL_RELEASES" regenerate data for archived releases as well
RELEASES = ACTIVE_RELEASES


class RetryError(Exception):
    pass


def retry(message: str = None, times: int = None, delay: int = 0):
    def wrap(f):
        def retry_wrapped(*args, **kwargs):
            if times is None:
                condition = lambda _: True
            else:
                condition = lambda x: x < times

            tries = 0

            while condition(tries):
                # noinspection PyBroadException
                try:
                    return f(*args, **kwargs)
                except KeyboardInterrupt:
                    print("Cancelling upon user request.")
                    raise
                except Exception:
                    tries += 1

                    if message is not None:
                        print(message, end="", flush=True)
                        print(f"{f.__name__}({args}, {kwargs})", flush=True)
                    if delay != 0:
                        time.sleep(delay)
            raise RetryError("Tried {} times, but failed.".format(times))

        return retry_wrapped

    return wrap


@retry(message="A request failed or timed out, retrying ...\n", times=None, delay=60)
def try_request(url: str, expected_keys: List[str]):
    ret = requests.get(url, timeout=120).json()
    for key in expected_keys:
        assert key in ret.keys()
    return ret


def do_builds() -> int:
    def per_release(release: str):
        builds = []
        page = 1
        pages = "?"

        while True:
            print(f"Builds: {release} page {page} / {pages}")
            data = try_request(
                f"{API_URL}/builds/?releases={release}&rows_per_page=100&page={page}",
                ["builds", "pages"]
            )

            builds.extend(data["builds"])

            page += 1
            pages = data["pages"]
            if page > pages:
                break

        with open(f"data/builds_{release.replace('-', '').lower()}.json", "w") as file:
            file.write(json.dumps(builds, indent=2))

        print(f"## Finished Builds: {release} ##")

    threads = []
    for rel in RELEASES:
        thread = threading.Thread(name=f"builds-{rel}", target=per_release, args=(rel,))
        threads.append(thread)
        thread.start()
        time.sleep(120)

    for thread in threads:
        thread.join()

    return 0


def do_comments() -> int:
    pages = try_request(f"{API_URL}/comments/?rows_per_page=50&page=1", ["pages"])["pages"]
    cpages = list(range(1, pages + 1))

    def per_page(page: int):
        print(f"Comments: page {page} / {pages}")
        data = try_request(f"{API_URL}/comments/?rows_per_page=50&page={page}", ["comments"])

        with open(f"data/comments_p{page}.json", "w") as jfile:
            jfile.write(json.dumps(data["comments"], indent=2))

    threads = []
    for cpage in cpages:
        thread = threading.Thread(name=f"comments-p{cpage}", target=per_page, args=(cpage,))
        threads.append(thread)
        thread.start()

        # do not DOS bodhi
        time.sleep(1)

    for thread in threads:
        thread.join()

    comments = []
    for cpage in cpages:
        with open(f"data/comments_p{cpage}.json") as file:
            comments.extend(json.loads(file.read()))
        os.remove(f"data/comments_p{cpage}.json")

    with open(f"data/comments.json", "w") as file:
        file.write(json.dumps(comments, indent=2))

    return 0


def do_composes() -> int:
    print(f"Composes ...")
    data = try_request(f"{API_URL}/composes/", ["composes"])

    new_composes = data["composes"]

    try:
        with open(f"data/composes.json") as file:
            old_composes = json.loads(file.read())
    except FileNotFoundError or json.JSONDecodeError:
        old_composes = []

    # for composes, there's no old data on the server, so keep it locally
    composes = old_composes + new_composes

    with open(f"data/composes.json", "w") as file:
        file.write(json.dumps(composes, indent=2))

    return 0


def do_overrides() -> int:
    def per_release(release: str):
        overrides = []
        page = 1
        pages = "?"

        while True:
            print(f"Overrides: {release} page {page} / {pages}")
            data = try_request(
                f"{API_URL}/overrides/?releases={release}&rows_per_page=25&page={page}",
                ["overrides", "pages"]
            )

            overrides.extend(data["overrides"])

            page += 1
            pages = data["pages"]
            if page > pages:
                break

        with open(f"data/overrides_{release.replace('-', '').lower()}.json", "w") as file:
            file.write(json.dumps(overrides, indent=2))

        print(f"## Finished Overrides: {release} ##")

    threads = []
    for rel in RELEASES:
        thread = threading.Thread(name=f"overrides-{rel}", target=per_release, args=(rel,))
        threads.append(thread)
        thread.start()
        time.sleep(300)

    for thread in threads:
        thread.join()

    return 0


def do_packages() -> int:
    packages = []
    page = 1
    pages = "?"

    while True:
        print(f"Packages: page {page} / {pages}")
        data = try_request(f"{API_URL}/packages/?rows_per_page=100&page={page}", ["packages", "pages"])

        packages.extend(data["packages"])

        page += 1
        pages = data["pages"]
        if page > pages:
            break

    with open(f"data/packages.json", "w") as file:
        file.write(json.dumps(packages, indent=2))

    return 0


def do_releases() -> int:
    releases = []
    page = 1
    pages = "?"

    while True:
        print(f"Releases: page {page} / {pages}")
        data = try_request(f"{API_URL}/releases/?rows_per_page=100&page={page}", ["releases", "pages"])

        releases.extend(data["releases"])

        page += 1
        pages = data["pages"]
        if page > pages:
            break

    with open(f"data/releases.json", "w") as file:
        file.write(json.dumps(releases, indent=2))

    return 0


def do_updates() -> int:
    def per_release(release: str):
        updates = []
        page = 1
        pages = "?"

        while True:
            print(f"Updates: {release} page {page} / {pages}")
            data = try_request(
                f"{API_URL}/updates/?releases={release}&rows_per_page=10&page={page}",
                ["updates", "pages"]
            )

            updates.extend(data["updates"])

            page += 1
            pages = data["pages"]
            if page > pages:
                break

        with open(f"data/updates_{release.replace('-', '').lower()}.json", "w") as file:
            file.write(json.dumps(updates, indent=2))

        print(f"## Finished Updates: {release} ##")

    threads = []
    for rel in RELEASES:
        thread = threading.Thread(name=f"updates-{rel}", target=per_release, args=(rel,))
        threads.append(thread)
        thread.start()
        time.sleep(600)

    for thread in threads:
        thread.join()

    return 0


def do_users() -> int:
    users = []
    page = 1
    pages = "?"

    while True:
        print(f"Users: page {page} / {pages}")
        data = try_request(f"{API_URL}/users/?rows_per_page=100&page={page}", ["users", "pages"])

        users.extend(data["users"])

        page += 1
        pages = data["pages"]
        if page > pages:
            break

    with open(f"data/users.json", "w") as file:
        file.write(json.dumps(users, indent=2))

    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("action", action="store", choices=[
        "all", "builds", "comments", "composes", "overrides", "packages", "releases", "updates", "users"
    ])
    parser.add_argument("--include-archived", action="store_const", const=True, default=False)

    args = vars(parser.parse_args())

    action = args["action"]
    include_archived = args["include_archived"]

    if include_archived:
        RELEASES.clear()
        RELEASES.extend(ALL_RELEASES)

    actions = {
        "builds": do_builds,
        "comments": do_comments,
        "composes": do_composes,
        "overrides": do_overrides,
        "packages": do_packages,
        "releases": do_releases,
        "updates": do_updates,
        "users": do_users,
    }

    if action == "all":
        ret = 0

        for fun in actions.values():
            ret += fun()

        return ret

    else:
        return actions[action]()


if __name__ == "__main__":
    try:
        exit(main())
    except KeyboardInterrupt:
        exit(0)

