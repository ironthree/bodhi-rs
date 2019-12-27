This directory contains tests to check if the data type definitions in
`src/data.rs` are correct and can successfully deserialize all JSON data that is
returned from bodhi.

These tests do **not** check any correctness or program logic, only whether all
server responses can be deserialized successfully. Tests that check logic or
more complex things are contained in `lib/tests` .

There are two python helper scripts (`download_data.py` and
`generate_tests.py`), which automate some of the boring stuff for getting the
latest data and automatically generating the test cases.

- test data for inactive / archived releases will not need to change (unless
  the schema of JSON responses from bodhi changes in the future)
- test data for active releases will be updated, but only seldomly, and after a
  release is archived, the data will not be updated (unless JSON schema changes)
- tests are generated automatically for all releases known to bodhi (though this
  list is maintained manually)

