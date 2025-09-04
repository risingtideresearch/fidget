/// Different rendering algorithms, implemented in Futhark.

/// A function to render a width by height image from the provided pure function.
///  We want to iterate from -1.0 to 1.0, along both axes.
///  (iota n) returns 0 ..< n, though.
///  So we transform our x and y values accordingly.
pub const naive_render: &'static str = "
def render(height: i64, width: i64, render_inner: (f64, f64, f64) -> f64): [][]f64 =
  let width_div = f64.i64 (width - 1) in
  let height_div = f64.i64 (height - 1) in
  map (\\(i: i64): [width]f64 ->
    let y = -((((f64.i64 i) / height_div) * 2.0) - 1.0) in
    map (\\(j: i64): f64 ->
      let x = ((((f64.i64 j) / width_div) * 2.0) - 1.0) in
      render_inner(x, y, 0.0))
      (iota width))
    (iota height)
";

// TODO: implement further rendering algorithms. parallelized?
