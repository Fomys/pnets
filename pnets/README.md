# PNETS

**Pnets is a framework for manipulating Petri nets**

- [crates.io](https://crates.io/crates/pnets)
- [docs.rs](https://docs.rs/pnets)

---

This crate provides an api for manipulating Petri nets. Two main structures are provided by this library:

- [`standard::Net`](https://docs.rs/pnets/latest/pnets/standard/struct.Net.html) - which allows to manipulate classical Petri
  net;
- [`timed::Net`](https://docs.rs/pnets/latest/pnets/timed/struct.Net.html) - which allows the manipulation of temporal Petri
  net.

In order to easily manipulate these nets this api provides the following elements:

- [`arc::Kind`](https://docs.rs/pnets/latest/pnets/arc/enum.Kind.html) - an enum of the different types of arcs that exist in
  a Petri net;
- [`Marking`](https://docs.rs/pnets/latest/pnets/struct.Marking.html) - a structure for manipulating hollow vectors;
- [`PlaceId`](https://docs.rs/pnets/latest/pnets/struct.PlaceId.html)
  and [`TransitionId`](https://docs.rs/pnets/latest/pnets/struct.TransitionId.html) - a type for indexing places and
  transitions in networks.
