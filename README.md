# Vest: Verified, Secure, High-Performance Parsing and Serialization for Rust

Anonymized code repository for Vest, a framework for high-performance,
formally verified binary parsers and serializers
that combines expressivity and ease of use with state-of-the-art correctness and security guarantees.

## Folder Structure

- `vest`: VestLib, the core library containing the primitive and higher-order
  combinator specifications, implementations, and their correctness and security
  proofs.
  - `src/properties.rs`: Key trait definitions and formal theorems that are
    discussed in **§3.2** and **§4.1** of the paper.
  - `src/buf_traits.rs`: Type abstractions for side-channel resilience that
    are discussed in **§4.2** of the paper.
  - `src/regular/`: *Detailed specifications, implementations, and proofs* of
    primitive and higher-order combinators described in **§3.1** of the paper.
    For each combinator, the `impl SpecCombinator` defines its specification;
    the `impl SecureSpecCombinator` proves its correctness and security
    properties; and the `impl Combinator` provides the efficient implementation
    of the combinator. Some major combinators can be found in the following files:
    - `src/regular/bytes.rs`: The `bytes::Fixed`, `bytes::Variable`, and `bytes::Tail` combinators.
    - `src/regular/uints.rs`: The `U8`, `U16Be`, `U16Be`, `U24Be`, `U24Le`,
      `U32Be`, `U32Le`, `U64Be`, and `U64Le` combinators.
    - `src/regular/modifiers.rs`: The combinators modifiers like `Mapped`, `Refined`, `AndThen`, `Cond`, etc.
    - `src/regular/sequence.rs`: The sequential combinators like `Pair`,
      `Preceded`, and `Terminated`.
    - `src/regular/repetition.rs`: The repetition combinators like `Repeat`
      and `RepeatN`.
    - `src/regular/variant.rs`: The variant combinators like `Choice` and `Opt`.
- `vest-dsl`: VestDSL, the domain-specific language (DSL) for defining binary
  formats.
  - `src/`: The VestDSL compiler, which parses, type-checks format definitions
    written in VestDSL, and generates efficient, verified, parsers and
    serializers leveraging VestLib.
  - `bitcoin/benches`: Benchmarks for the Bitcoin block and transaction formats (**§6.1**).
  - `tls/benches`: Benchmarks for the TLS handshake messages (**§6.2**).
  - `wasm/benches`: Benchmarks for the WebAssembly binary format (**§6.3**).
- `tools`: Miscellaneous tools and scripts that are used benchmarking.


## Usage

Make sure you have [Rust](https://www.rust-lang.org/tools/install) and [Verus](https://github.com/verus-lang/verus/blob/main/INSTALL.md) properly installed. Then, clone the repository and run:

- To verify and compile the entire `vest` crate:

```sh
cd vest
make
```

- To use the VestDSL:

```sh
cd vest-dsl
cargo run path/to/your/file.vest
```

- Or you can build the `vest-dsl` crate and use the binary directly:

```sh
cd vest-dsl
cargo build --release
./target/release/vest-dsl --help
Usage: vest-dsl [OPTIONS] <VEST_FILE>

Arguments:
  <VEST_FILE>  Name or directory of the vest file

Options:
  -o, --output <OUTPUT>  Name of the output verus file
  -h, --help             Print help
  -V, --version          Print version
```

VestDSL would generate a `<OUTPUT>.rs` file containing the datatype definitions, as well as the specifications, proofs, and implementations of the parsers and serializers for the defined format.
Once the `vest` crate is added as a dependency in the `Cargo.toml` file, users can include this file as a module in their Rust projects (see [Vest Bitcoin](./vest-dsl/bitcoin/src/main.rs) for an example) and use the generated parsers and serializers.

- To *verify* the case studies, simply run the `make` command in the respective case study directories:

```sh
cd vest-dsl/bitcoin
make
```

- To run the benchmarks, you can use the `cargo bench` command in the respective benchmark directories:

```sh
cd vest-dsl/bitcoin/benches
cargo bench
```

> NOTE: To run the benchmark for the Bitcoin block format, you first need to download the [sampled_blocks.txt](./vest-dsl/bitcoin/benches/data/sampled_blocks.txt) file, which contains the sampled Bitcoin blocks used in the benchmark. You can download it from [here](...).
