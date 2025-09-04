# futhark

Code for Futhark integration into Fidget.

Originally, the plan was to support Futhark in a more principled fashion by using one of the existing Futhark integration crates:
- [cargo-futhark](https://github.com/luleyleo/cargo-futhark)
- [genfut](https://github.com/Erk-/genfut)
- [futhark-bindgen](https://github.com/zshipko/futhark-bindgen)

Unfortunately, these all focus on integrating *static* Futhark code with Rust. We want to support *dynamic* Futhark code: being able to generate our Futhark on-the-fly as a user might update a sketch.

So the code here rolls its own integration. At some point, it would be better to implement a proper FFI. The subprocesses are only really usable for prototyping and the parser is jank. (Though, we would only be parsing 2-arrays and 3-arrays of floats, so it's not so bad. Just ideally should be unnecessary with an FFI.)
