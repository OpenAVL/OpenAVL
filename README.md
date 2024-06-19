# OpenAVL
## Building the Project
You will need to have [rust](https://www.rust-lang.org/) and [cargo](https://crates.io/) installed to build this project.

To run all executable packages (and compile them if needed) use:
```
cargo run
```

To compile everything use:
```
cargo build
```

To run all tests use:
```
cargo test
```

To compile/run/test only a specific package use:
```
cargo <build/run/test> -p <your package name>
```
e.g.
```
cargo build -p openavl
cargo run -p test_everything
```