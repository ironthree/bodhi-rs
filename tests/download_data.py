#!/usr/bin/python3 -OO

import requests
import json
import threading
import time

API_URL = "https://bodhi.fedoraproject.org"

ALL_RELEASES = [
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
    "EPEL-8",
    "EPEL-8M",
    "EPEL-7",
    "EL-6",
    "EL-5",
]

ACTIVE_RELEASES = [
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
    "EPEL-8",
    "EPEL-8M",
    "EPEL-7",
    "EL-6",
]

# change this to "ALL_RELEASES" regenerate data for archived releases as well
RELEASES = ACTIVE_RELEASES

# TODO: download data only for active releases


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
                    if delay != 0:
                        time.sleep(delay)
            raise RetryError("Tried {} times, but failed.".format(times))

        return retry_wrapped

    return wrap


@retry(message=None, times=3, delay=5)
def try_request(url: str):
    ret = requests.get(url, timeout=120)
    return ret


def do_builds() -> int:
    def per_release(release: str):
        builds = []
        page = 1
        pages = "?"

        while True:
            print(f"Builds: {release} page {page} / {pages}")
            ret = try_request(f"{API_URL}/builds/?releases={release}&rows_per_page=100&page={page}")
            data = ret.json()

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

    for thread in threads:
        thread.join()

    return 0


def do_overrides() -> int:
    def per_release(release: str):
        overrides = []
        page = 1
        pages = "?"

        while True:
            print(f"Overrides: {release} page {page} / {pages}")
            ret = try_request(f"{API_URL}/overrides/?releases={release}&rows_per_page=100&page={page}")
            data = ret.json()

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

    for thread in threads:
        thread.join()

    return 0


def do_packages() -> int:
    packages = []
    page = 1
    pages = "?"

    while True:
        print(f"Packages: page {page} / {pages}")
        ret = try_request(f"{API_URL}/packages/?rows_per_page=100&page={page}")
        data = ret.json()

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
        ret = try_request(f"{API_URL}/releases/?rows_per_page=100&page={page}")
        data = ret.json()

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
            ret = try_request(f"{API_URL}/updates/?releases={release}&rows_per_page=100&page={page}")
            data = ret.json()

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

    for thread in threads:
        thread.join()

    return 0


def do_users() -> int:
    users = []
    page = 1
    pages = "?"

    while True:
        print(f"Users: page {page} / {pages}")
        ret = try_request(f"{API_URL}/users/?rows_per_page=100&page={page}")
        data = ret.json()

        users.extend(data["users"])

        page += 1
        pages = data["pages"]
        if page > pages:
            break

    with open(f"data/users.json", "w") as file:
        file.write(json.dumps(users, indent=2))

    return 0


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
