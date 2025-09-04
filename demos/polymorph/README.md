# demos/polymorph

A demo crate using the Futhark integration to render `.vm` files to the PPM file format.

Usage: `cargo run -ppolymorph`

There is currently strange behavior going on with the generated Futhark, especially evident in the prospero file. There's a very long chain of `min` and `max` comparisons that I think can be optimized better. Two things to explore:
- Why is Fidget producing these comparisons in the first place? Is it really the most optimal form?
- Can we compile these comparison chains more efficiently? A branching `if` statement?
