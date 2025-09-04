-- Using Futhark as a compilation target from Rust
-- #x, #y, #z: the inputs passed in
-- #const: a constant value
-- everything else: native futhark functions
-- https://github.com/diku-dk/futhark/issues/1562

type unop_tag =
  #recip | #abs | #neg | #sqrt | #square | #floor | #ceil | #round |
  #sin | #cos | #tan | #asin | #acos | #atan | #ln | #not | #exp

type binop_tag =
  #add | #mul | #min | #max | #div | #atan2 | #sub | #compare | #mod | #and | #or

-- A helper function for Rust to use when targeting Futhark.
-- Can't return a first class function. Defunctionalization issue?
#[inline]
def eval_unop(tag: unop_tag, arg: f64): f64 =
  match tag
  case #recip -> 1.0 / arg
  case #abs -> f64.abs arg
  case #neg -> f64.neg arg
  case #sqrt -> f64.sqrt arg
  case #square -> arg ** 2
  case #floor -> f64.floor arg
  case #ceil -> f64.ceil arg
  case #round -> f64.round arg
  case #sin -> f64.sin arg
  case #cos -> f64.cos arg
  case #tan -> f64.tan arg
  case #asin -> f64.asin arg
  case #acos -> f64.acos arg
  case #atan -> f64.atan arg
  case #ln -> f64.log arg
  case #not -> if arg == 0.0 then 1.0 else 0.0
  case #exp -> f64.exp arg

-- A helper function for Rust to use when targeting Futhark.
-- This is only useful if Futhark is smart enough to inline the (static) tag.
-- If it's not, we should make it so...
#[inline]
def eval_binop(tag: binop_tag, l: f64, r: f64): f64 =
  match tag
  case #add -> l + r
  case #mul -> l * r
  case #min -> f64.min l r
  case #max -> f64.max l r
  case #div -> l / r
  case #atan2 -> f64.atan2 l r
  case #sub -> l - r
  case #compare ->
    if l < r then -1.0
    else if l == r then 0.0
    else if l > r then 1.0
    else f64.nan
  case #mod -> l % r -- rem_euclid
  case #and -> if l == 0.0 then l else r
  case #or -> if l != 0.0 then l else r

