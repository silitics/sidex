# List the available recipes.
default:
    @just --list


# Run the Sidex command line tool.
run *ARGS:
    cd {{invocation_directory()}} && cargo run --bin sidex -- {{ARGS}}


# Format all files using Rustfmt nightly.
fmt:
    cargo +nightly fmt


# Generate the API documentation using Rustdoc.
doc:
    cargo +nightly doc --lib --document-private-items


# Run all tests.
test:
    cargo test
