# GA - A Genetic Algorithm Framework
This library implements a generic framework for developing genetic algorithms
in Rust.

---
## Usage
To implement a genetic algorithm with this framework the following steps are
needed:

### Implement Traits
The framework heavily relies on traits as interface definitions, to be
as indepented from the problem instance as possible. Therefore, the user of
this library must first implement certain traits for the data structures, that
represent the problem that needs to be solved.

#### The `Context` Trait
The [`encoding::Context`] trait is a so called *marker* trait, which specifies
the implementing struct to be the *context* for a genetic algorithm.
But what is a context? The context is a `struct` of arbitrary data, which
is automatically injected into other trait methods, to give the user of
this framework flexibility in modeling a given problem.

An example for data that goes into the context is a random number generator
that depicts a special, non-uniform probability distribution. Such a number
generator might be used in various parts of the algorithm and is not really
part of the genotype or phenotype. Therefore it can be stored in the context,
as the context is automatically injected into all relevant trait methods.

Example:
```rust ,ignore
pub struct Context {
    // Here goes some special data.
}

// Implementation of the context trait. The implementation block is empty,
// because the trait is a marker trait!
impl ga::encoding::Context for Context {}
```


#### The `ObjectiveValue` Trait
The [`encoding::ObjectiveValue`] trait must be implemented by the data structure
of an encoding, which represents the objective value (e.g., the fitness or
cost) of a potential solution.

**Attention:** A current limitation of this trait is, that the implementing
data must be convertable into an `usize`.


#### The `Genotype` Trait
The [`encoding::Genotype`] trait must be implemented by the data structure,
that represents the genotype of an encoding. Its trait definition primarily
contains a method, which is used by the algorithm framework to initialize a
population in the beginning of the algorithm execution.

The example below shows, that implementing the genotype trait requires a
context as type parameter. That is due to the `generate` method of the
genotype trait providing the context to the user of the framework, for
accessing data stored in the user-defined context struct.

Example:
```rust ,ignore
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Genotype(Vec<u8>);

impl trait Genotype<Context> for Genotype {
    fn generate(amount: usize, ctx: &Context) -> Vec<Self> { ... }
    ...
}
```

#### The `Phenotype` Trait
Similar to the genotype, the [`encoding::Phenotype`] trait is implemented for
data structures that represent the phenotype of a genetic algorithm encoding.
This trait defines methods for deriving a phenotype from a given genotype,
as well as evaluating the phenotype to yield an objective value.

Hence, the phenotype requires multiple generic type parameters, to be as
generic as possible. The example below shows how this trait can be implemented:

```rust ,ignore
#[derive(Clone, Debug)]
pub struct Phenotype { ... }

impl ga::encoding::Phenotype<ObjVal, Context, Genotype> for Phenotype {
    fn derive(&self, geno: &Genotype, ctx: &Context) -> Self { ... }
    fn evaluate(&self, ctx: &Context) -> ObjVal { ... }
}
```

### Genetic Operators
The genetic operators "crossover" and "mutation" cannot be provided by
the framework, because they heavily depend on the data structure of the
encoding.

Therefore, the framework provides the traits
[`operators::Crossover`] and [`operators::Mutation] which need to
be implemented possibly by an enum, which unifies all supported crossover
(or mutation operators respectively).

Apart from those traits, the framework also provides implementations for some
of the most commonly used crossover and mutation methods, which can be
used to implement the crossover and mutation traits or as a reference, to
implement custom operators.
The prebuild operators are:
-   Crossover
    -   Single Point            [`operators::crossover::single_point`]
    -   Multi Point             [`operators::crossover::multi_point`]
    -   Ordered                 [`operators::crossover::ordered`]
    -   Partially Mapped (PMX)  [`operators::crossover::pmx`]
    -   Uniform                 [`operators::crossover::uniform`]
-   Mutation
    -   Random                  [`operators::mutation::randomize_single_dist`]
        and                     [`operators::mutation::randomize_multi_dist`]
    -   Swap (multiple variants) `operators::mutation::swap_*`

### Preparing the Algorithm
With the above traits implemented, we can start assembling the encoding,
parameters and dynamics of the algorithm.

First, the context and phenotype must be initialized. The methods of
initialization for both types are not covered by any trait and are therefore
part of the user's custom implementation. In the example below, the
`instance` variable represents the problem instance, which is used by the
initialization functions of the context and phenotype, to set them up
correctly:
```rust ,ignore
let ctx = Context::init(&instance);
let ph = Phenotype::blueprint(&instance);
```

The encoding is created by using a builder pattern as shown below:
```rust ,ignore
let encoding = ga::encoding::Builder::new()
    .set_context(ctx)
    .set_phenotype(ph)
    .build();
```

Similarly, the parameters of the genetic algorithm are defined as follows.
The crossover and mutation enums are thereby implemented by the user of the
framework, whereas the selection, rejection, replacement and termination
enums are provided by the framework.
```rust ,ignore
let parameters = ga::parameters::Builder::for_encoding(&encoding)
    .set_population_size(1_000)
    .set_crossover_rate(None)
    .set_mutation_rate(0.010)
    .set_selection(Select::LinearRank(2.0))
    .set_crossover(Crossover::Trade(1))
    .set_mutation(Mutation::Trade)
    .set_rejection(Reject::None)
    .set_replacement(Replace::EliteAbsolute(1))
    .set_termination(Terminate::GenOrOv(500_000, 0.into()))
    .build();
```

If the usage of self-parameterization (aka. *dynamics*) is desired, they
are defined as follows:
```rust ,ignore
let dynamics = ga::dynamics::Builder::for_parameters(&parameters)
    .set(vec![
        // Dynamic methods go here
    ])
    .build();
```

Finally, the algorithm can be build:
```rust ,ignore
let alg = ga::Builder::new()
    .set_encoding(encoding)
    .set_parameters(parameters)
    .set_dynamics(Some(dynamics)) // alternatively: set_dynamics::<()>(None)
    .set_custom_logger::<()>(None)
    .build();
```

### Executing the Algorithm
Executing the algorithm is now as easy as calling one function:
```rust ,ignore
let report = alg.run();
```
After the algorithm terminates it returns a [`report::Report`] which
contains not only the individuals of the final generation (incl. their
objective values), but also metrics collected during the runtime of the
algorithm as well as a detailed `log`, containing information about each
generation to be used for post-analyzation of the run.

---
## Advanced Usage
This section explains more advanced use cases of the framework and how
to implement them.

### Self-Parameterization
The framework supports self-parameterization of the genetic algorithms, by
simply implementing the [`dynamics::Dynamic`] trait. This trait is usually
implemented by enums, which represent a set of self-parameterization methods.

This trait defines three methods that are crucial for enabling self-
parameterization:
1.  `setup`: The setup method can be used to modify algorithm parameters, the
    context or even alter the runtime data struct, if this is necessary.
    If none of these modifications are needed, the function body can simply
    be left empty.

2.  `exec`: This method is responsible for actually performing the
    self-parameterization. It is called by the algorithm execution within the
    framework at the end of each generation, and has the ability to modify
    the algorithms parameters as well as its context.

    Furthermore, the `exec` method has access to the `rerun_logger` instance
    (only if the `log_dynamics` feature of this crate is enabled), to
    send logs to Rerun, which are specific to this method of
    self-parameterization.

3.  `identifier`: The identifier must be a unique string which identifies
    this dynamic among the other dynamics, and can therefore be used for
    referring to a method of self-parameterization for example in diagrams.

An example of how to implement such self-parameterization methos (= dynamics),
check out the implementations in `alg_11` and `alg_12`. They can be found
-   here `lib/alg_11/dynamics/mod.rs` and
-   here `lib/alg_12/dynamics/mod.rs`.

### Logging with Rerun

### Custom Logger


---
## Crate Features
-   `cache`: This feature enables a cache which stores the results of objective
    value calculations. If the same chromosome is "discovered" multiple times,
    this cache allows skipping the evaluation of the chromosome.

-   `no_stdout_log`: This feature disables console logs during the algorithm
    execution and is **meant to be used** in cases where maximum performance
    is crucial.

-   `rerun_logger`: This feature enables logging to "Rerun". Enabling this
    feature some basic metrics are automatically logged to "Rerun". For logging
    more advanced data, see the features below.

-   `log_pop_stats`: Log population statistics like population size, elite size,
    selection size and number of distinct selections **to Rerun**.  
    *This automatically enables the `rerun_logger` feature.*

-   `log_cache_hits`: Log cache hits and misses.  
    *This feature automatically enables the `cache` and `rerun_logger` feature.*

-   `log_runtimes`: Log runtime measurements for performance insights.  
    *This feature automatically enables the `rerun_logger` feature.*

-   `log_dynamics`: Enabling this feature makes the `rerun_logger` part of the
    function signatures of the `Dynamics` trait. This allows users, who
    implement dynamic behaviour (by implementing the `Dynamics` trait), to log
    their custom metrics to rerun.  
    *This feature automatically enables the `rerun_logger` feature.*

---
