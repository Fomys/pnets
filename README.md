# PNETS Libraries & Shrink Tool

This repository includes a set of libraries to manipulate standard and timed Petri net in rust.
This work is part of a global project from the VERTICS group at LAAS/CNRS.

# License

Copyright 2021 Louis Chauvet

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

# Content

## `pnets` library

This library is the core to manipulate Petri nets. It has the support for standard net and timed net.

### Internals

The Petri net is stored with adjacency lists, each nodes knows its connected transitions and each transitions knows
its nodes. This allows a low memory footprint and a fast way to iterate over connections.

Places and transitions store their string id and label, but they are internally indexed by using typed integer to avoid
unintentional mix between places and transitions with no runtime overhead.

## `pnets_pnml` and `pnets_tina` libraries

The purpose of these libraries is to parse and export Petri nets to the `.pnml` and `.net` formats (textual format for Petri nets described in [the Tina man pages](http://projects.laas.fr/tina/manuals/formats.html)).

## `pnets_shrink` library

This library provides methods and traits to reduce Petri nets.
It provides all rules from [STTT](https://todoref.fr) and new rules created specially to reduce some nets from Petri net contest.

### Internals

This crate provides the following reduction rules:
- Simple chain agglomeration
- Simple loop agglomeration
- Identity places and transitions
- Parallel places and parallel transitions
- SourceSink reduction
- Invariant reduction (by calling `struct` from the Tina toolbox)

This crate also provide special reductions, created for specific models from the Model Checking Contest ([MCC](https://mcc.lip6.fr/)):
- Pseudo start reduction (Election2020)
- Weight simplification (Election2020)
- RL reduction (ViralEpidemic)

You can found all information to use the library on [docs.rs](https://docs.rs/pnets_shrink/0.1.0/pnets_shrink/).

Following a small example to use two reductions:


```rust
use pnets_tina;
use pnets_shrink::modifications::Modification;
use pnets_shrink::reducers::standard::{
    SimpleChainReducer,
    SimpleLoopAgglomeration
};
use pnets_shrink::reducers::{
    ChainReducer,
    LoopReducer,
};
use pnets_tina::ExporterBuilder;

pub fn main() {
    let mut net = pnets_tina::Parser::new(BufReader::new(File::open("mynet.net")?)).parse()?.into();
    let mut modifications = vec![];
    LoopReducer::<
        _,
        ChainReducer::<
            _,
            SimpleChainAgglomeration,
            SimpleLoopAgglomeration
        >
    >::reduce(&mut net, &mut modifications);
    
    ExporterBuilder::new(stdout())
        .build()
        .export(&net.into())?;
}
```

## The `shrink` Tool

This tool uses the previous libraries to reduce Petri nets. It produces exactly the same format as the `reduce` tool
developed by the VERTICS team ([tina](http://projects.laas.fr/tina/home.php)).

### Usage

#### Input

The tool can read `.pnml` and `.net` formats from file or stdin.

- [optional, default: -] `--input [file]`/`-i [file]` - file to read,  use `-` to read from stdin
- [optional, default: guess] `--format [net/pnml/guess]` - input format, if the value is `guess` the detection is based on the file extension

#### Output

The tools use the `.net` format as output, you can set some options to tweak its output:
- [optional, default: absent] `--clean` - remove all transitions and places that are disconnected
- [optional, default: absent] `--equations` - print the equations as comments (`#`)

#### Reductions

By default no reductions are applied, you can enable them by groups:
- [optional, default: absent] `--compact` - enable compact reductions: `SimpleLoopAgglomeration` and `SimpleChainReducer`
- [optional, default: absent] `--redundant` - enable redundant reductions: `InvariantReducer`, `ParallelPlaceReducer`, `ParallelTransitionReducer`, `IdentityPlaceReducer`, `IdentityTransitionReducer` and `R7Reducer`
- [optional, default: absent] `--extra` - enable extra reductions: `WeightSimplification`, `RLReducer`, `PseudoStart`
- [optional, default: absent] `--struct` - enable invariant reduction by calling struct, `struct` executable and `4ti2` must be installed.


## Executable `pnets_print`

A simple example of the use of libraries `pnets` and `pnets_tina` to read and count the number of places and transitions
in a Petri net.

# Installation

## Libraries
To use the libraries add them to your `Cargo.toml` file:
```toml
[dependencies]
# pnets core library
pnets = "0.1"
# pnets tina parser
pnets_tina = "0.1"
# pnets pnml parser
pnets_pnml = "0.1"
# pnets shrink library
pnets_shrink = "0.1"
```

## Executables

### Get from cargo

You just have to install it as every cargo executable.

```shell
# Install pnets_shrink
cargo install pnets_shrink
# Run pnets_shrink
pnets_shrink --help
```

### Build from source

Clone this repository and build the executable:
```shell
git clone https://github.com/fomys/pnets
cd pnets
# Build pnets_shrink
cargo build --release
# Run pnets_shrink
target/release/pnets_shrink --help
```

#### Build with musl

To allow a better compatibility, it is possible to build against static musl instead of libc, the build commands become:
```shell
# If you don't have the toolchain installed
rustup target add x86_64-unknown-linux-musl
# Build pnets_shrink
RUSTFLAGS="-C link-arg=-s" cargo build --release --target=x86_64-unknown-linux-musl
# Run pnets_shrink
target/x86_64-unknown-linux-musl/release/pnets_shrink --help
```
With this the executable is statically linked and can be run on any linux machine.

#### Running tests

You can run tests with the command `cargo test`.