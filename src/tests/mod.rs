/// use the fedora bodhi instance for running tests
const SERVER_URL: &str = "https://bodhi.fedoraproject.org";

mod builds;
mod comments;
mod overrides;
mod packages;
mod releases;
mod stacks;
mod updates;
mod users;

// TODO: move things from main.rs here
// TODO: query *everything* from bodhi and make sure deserialization works
