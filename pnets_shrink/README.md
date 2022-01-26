# PNETS shrink

**PNETS shrink is a library which reduce Petri nets according while
keeping some properties on the net**

- [crates.io](https://crates.io/crates/pnets_shrink)
- [docs.rs](https://docs.rs/pnets_shrink)
---

This crate provides an api for creating reductions on Petri nets and integrating some of the reductions described in the
paper [Counting Petri net markings from reduction equations](https://doi.org/10.1007/s10009-019-00519-1).

This crate provides an api for creating reductions on Petri nets and integrating some of the reductions described in the publication [STTT](https://doi.org/10.1007/s10009-019-00519-1).

This crate provides two modules:
- A management of modifications using [modifications];
- Management of reduction algorithms using [reducers].

The [modifications] module gathers the different modifications that
can be made to a net while preserving certain properties.

The [reducers] module provides three features: [Reduce],
[PlaceReduce] and [TransitionReduce] which allow all reduction
algorithms to have a common interface.

There are also generic reduction algorithms that make it easier to
handle chaining and looping of reductions: [IdentityReducer],
[LoopReducer], [SmartReducer] and [ChainReducer].

This version of the library also provides reductions that apply to
standard Petri nets within the [reductions::standard] module.

## Usage

```rust
use pnets::standard::Net;
use pnets_shrink::reducers::standard::SimpleChainReducer;

fn main() {
    // Load a standard Petri net from stdin
    let mut net = Net::from(pnets_tina::Parser::new(BufReader::new(io::stdin())).parse()?);
    
    // Application of the reduction
    let mut modifications = vec![];
    SimpleChainReducer::reduce(&mut net, &mut modifications);

    // Auto naming is needed to allow fill name for auto-created places
    net.auto_name();
    
    // Display modifications
    println!("{:?}", modifications);
    // Display new net on stdout
    ExporterBuilder::new(file)
        .build()
        .export(net)?;
}
```


[modifications]: https://docs.rs/pnets_shrink/latest/pnets_shrink/modifications/index.html
[reducers]: https://docs.rs/pnets_shrink/latest/pnets_shrink/reducers/index.html
[Reduce]: https://docs.rs/pnets_shrink/latest/pnets_shrink/reducers/trait.Reduce.html
[PlaceReduce]: https://docs.rs/pnets_shrink/latest/pnets_shrink/reducers/trait.PlaceReduce.html
[TransitionReduce]: https://docs.rs/pnets_shrink/latest/pnets_shrink/reducers/trait.TransitionReduce.html
[IdentityReducer]: https://docs.rs/pnets_shrink/latest/pnets_shrink/reducers/struct.IdentityReducer.html
[LoopReducer]: https://docs.rs/pnets_shrink/latest/pnets_shrink/reducers/struct.LoopReducer.html
[SmartReducer]: https://docs.rs/pnets_shrink/latest/pnets_shrink/reducers/struct.SmartReducer.html
[ChainReducer]: https://docs.rs/pnets_shrink/latest/pnets_shrink/reducers/struct.ChainReducer.html
[reductions::standard]: https://docs.rs/pnets_shrink/latest/pnets_shrink/reducers/standard/index.html
