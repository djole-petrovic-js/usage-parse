*Simple project in Rust to parse all usage log files and update user usage data.*

The program first parses all command-line interface arguments.
After that, it parses every log file and assembles the aggregate usage struct.
Currently, nothing is done with the aggregation; it is just printed. In a production scenario, it should update a database, call a webhook, or perform another action.

This project is a learning experience, so DO NOT use this code in production.

*Compilation*

To compile the program and run it, use these commands:

```
cargo build --release

./target/release/usage-parse --log_dir=logs
```

*Development*

To run the program during development, use this command:

```
cargo run -- --log_dir=logs
```

*Testing*

To run all the tests, use this command:

```
cargo test
```