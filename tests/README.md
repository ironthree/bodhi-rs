This directory contains tests to check if the data type definitions in
`src/data.rs` are correct and can successfully deserialize all JSON data that is
returned from bodhi.

These tests do **not** check any correctness or program logic, only whether all
server responses can be deserialized successfully. Tests that check logic or
more complex things are contained in `lib/tests` .

There are two python helper scripts (`download_data.py` and
`generate_tests.py`), which automate some boring stuff for getting the latest
data and automatically generating the test cases.

However, since the test data is rather large (multiple files with up to ~150
MB), these files are not committed into git. Also, test data for inactive /
archived releases will not need to change (unless the schema of JSON responses
from bodhi changes in the future).
