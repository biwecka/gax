# 🧬 GAX - Genetic Algorithms for XHSTT 🧬
This repository contains all the genetic algorithms and auxiliary crates I
developed for my master's thesis with the title:
> **Development of a self-parameterizing Genetic Algorithm:**
> A Case Study on High School Timetabling

Check out all my repositories related to this topic:
[🇧🧬 Genetic Algoritms for XHSTT](https://github.com/stars/biwecka/lists/genetic-alg-for-xhstt)


## Repository Overview
The **main** crates in this repo are:
-   `bin/solver`: Binary used for executing any XHSTT solving algorithm on a
    given problem instance. The used algorithm and problem instance must be
    configured in `main.rs`.
-   `bin/auto_runner`: Binary to execute `alg_11` or `alg_12` with
    given configurations in an infinite loop - perfect for capturing a lot
    of data about the algorithms.
-   `lib/alg_11`: A XHSTT solver with *direct* encoding.
-   `lib/alg_12`: A XHSTT solver with *indirect* encoding.


Furthermore, this repo contains the following crates:
-   `lib/ga`: Framework for developing genetic algorithms in Rust.
-   `lib/xhstt`: Library to read and write XHSTT `xml` files and query their
    included datastructures
-   `lib/bits`: High performance bit vector libraray.
-   `bin/eggholder`: Demo of self-parameterization on eggholder function.
-   and many more...


## Repository Structure
```sh
archive             # Old attempts of developing a genetic algorithms.
assets              # XHSTT instance (XML files) and calculated solutions.
bin/                # Binaries
├── solver
├── auto_runner
└── ...
docs                # Markdown documentation
examples/           # Examples
├── bits_demo       # Showcase of the `bits` library.
└── ...
lib/                # Libraries
├── alg_11          # GA with direct encoding
├── alg_12          # GA with indirect encoding
├── ga              # GA framework
├── xhstt           # Library for reading, writing and querying XHSTT instances
├── bits            # High-performance bit vector library.
└── ...
```

<!-- Usage ----------------------------------------------------------------- -->
# Usage
## Using the `solver`
To run any of the contained algorithms follow these steps:
1.  Select a XHSTT problem instance by modifying the following line:
    ```rust
    // bin/solver/main.rs
    fn main() {
        let selection = Archives::X2014a(X2014a::Hdtt4);
        ...
    }
    ```

2.  Choose an algorithm:
    1.  Add its dependency to `bin/solver/Cargo.toml` like so:
        ```toml
        [dependencies]
        alg_12 = { workspace = true }
        ```

    2.  Use the algorithm to solve the problem by editing the following line:
        ```rust
        // bin/solver/main.rs
        fn main() {
            ...
            let solution_events = alg_12::run(instance.clone());
            ...
        }
        ```

3.  Run the solver with the following command: `cargo rr solver`
    (alias for `cargo run --release -p solver`)


## Using the `auto_runner`
The auto runner binary does not only execute a selected algorithm over and
over again, with multiple configurations, it also **writes** the captured data
**to another git repository** and automatically **commits and pushes** the
changes whenever it writes new data to disk.
In my thesis I used the [GAX Plots](https://github.com/biwecka/gax-plots)
repository to store and plot all data collected by the auto runner executions.
*Feel free to take inspiration from this repo when collecting your own*
*algorithm execution data or forking the repo to extend its datasets.*

Moreover the auto runner integrates the "Pushover" notification service, to be
notified on a phone if an error occurs, which comes in very handy for letting
the auto runner execute algorithms unsupervised for a long time.

To use the auto runner, the following steps are needed:
1.  Choose an algorithm (`alg_11` or `alg_12`):
    1.  Enable the correct executor module:
        ```rust
        // bin/auto_runner/main.rs
        mod executor_alg_12;
        use executor_alg_12::ExecutorAlg12;
        ```

    2.  Use the imported algorithm executor:
        ```rust
        // bin/auto_runner/main.rs
        fn main() {
            ...
            let mut exec = ExecutorAlg12::new(env);
            ...
        }
        ```

2.  Define algorithm configurations in
    `bin/auto_runner/executor_alg_12/configs.rs`
    (or `executor_alg_11/configs.rs` respectively).

3.  Create a `.env` file in the repository root and configure the following
    environment variables:
    ```sh
    PLOTS_REPO=/absolute/path/to/gax-plots/repo
    DATA_DIR=/absolute/path/to/gax-plots/repo/data
    GIT_USERNAME=john_doe
    GIT_PASSWORD=password_or_access_token

    PUSHOVER_API_KEY=app_api_key
    PUSHOVER_USER_KEY=user_key
    ```

4.  `cargo rr auto_runner` starts the auto runner

5.  Pressing `Ctrl`+`C` stops the auto runner gracefully, by waiting for the
    current algorithm execution to terminate, writing, committing and pushing
    its result and finally stopping the auto runner execution.


<!-- Documentation --------------------------------------------------------- -->
# Documentation
For technical documentation and implementation insights check out the
markdown documentation [here](./docs/main.md).


<!-- ----------------------------------------------------------------------- -->
