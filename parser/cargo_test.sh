# This shell script runs the tests with any arguments you give it
# This is necessary because of pyo3 mucking about
cargo test --no-default-features $*
