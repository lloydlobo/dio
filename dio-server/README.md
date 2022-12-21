# Dio

- [Dio](#dio)
  - [Usage](#usage)
  - [Install](#install)
  - [Dev](#dev)
    - [Watch Development Mode](#watch-development-mode)
    - [Manage modules](#manage-modules)
    - [Build](#build)
    - [Clippy Linting](#clippy-linting)
    - [Test](#test)
    - [CI](#ci)
      - [Audit](#audit)
      - [Code coverage x64 Linux](#code-coverage-x64-linux)
    - [Database](#database)
      - [Install MongoDB](#install-mongodb)
      - [Set up environment variable](#set-up-environment-variable)
      - [Run in watch mode](#run-in-watch-mode)
  - [Resources](#resources)

## Usage

TODO

## Install

Install the binary.

```bash
cargo install --path .
```

NOTE: Please make sure you have cargo installed via rustup.

## Dev

### Watch Development Mode

```bash
cargo watch -c -q -w  ./src -x run
cargo watch -c -q -w  ./src -x "run -- --option facts --key 4"
```

```sh
-c clear terminal
-q supress cargo watch output
-w only focus on sourcedirectory
```

### Manage modules

```bash
cargo-modules generate tree <OPTIONS>

<OPTIONS>
```

### Build

```bash
cargo make --makefile build.toml build-flow
```

### Clippy Linting

```bash
bacon clippy-all
```

### Test

```bash
cargo nextest run
```

### CI

#### Audit

```bash
cargo audit
```

#### Code coverage x64 Linux

```bash
cargo tarpaulin -v
```

```bash
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Dec 21 02:00:02.157  INFO cargo_tarpaulin::report: Coverage Results:
|| Uncovered Lines:
|| src/cronjob.rs: 11, 13-15, 17, 20-21, 39
|| src/dioerrors.rs: 8-10, 30-33
|| src/main.rs: 54-56, 60-61, 63-65, 67, 70-73, 77, 86, 97-102, 110-112, 114-116, 120-122, 124-126, 134-137, 139-141, 151-154, 156-158, 171, 177, 179, 181-183, 185, 187, 190, 196, 198, 200-202, 204, 206, 210-218, 220, 222-227
|| src/testcases.rs: 4-9, 11-15, 20-25, 27-31
|| Tested/Total Lines:
|| src/cronjob.rs: 0/8
|| src/dioerrors.rs: 0/7
|| src/main.rs: 0/79
|| src/testcases.rs: 0/22
||
0.00% coverage, 0/116 lines covered
```

### Database

#### Install MongoDB

Visit [MongoDB Download Center] for instructions.

- [Check example](https://github.com/actix/examples/blob/master/databases/mongodb/src/main.rs)

#### Set up environment variable

The example code creates a client with the URI set by the `MONGODB_URI`
environment variable. The default URI for a standalone mongod running on
localhost is "mongodb://localhost:27017". For more information on MongoDB URIs,
visit the [connection string](https://docs.mongodb.com/manual/reference/connection-string/) entry in the MongoDB manual.

#### Run in watch mode

To execute the code, run cargo run in the repository's root directory.

## Resources

[Resource](https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/)
