// This module provides functions for rendering to PNM image file (via a ByteArray).

/// Renders its input (a 2-array of f64 values) to a PPM file based on a provided threshold
pub fn to_pbm(render: &Vec<Vec<f64>>, threshold: f64) -> Vec<u8> {
  let threshold = threshold;
  let height = render.len();
  let width = render.get(0).unwrap_or(&vec![]).len();

  let mut buf = Vec::new();
  buf.append(&mut "P1".as_bytes().into());
  buf.push('\n' as u8);
  buf.append(&mut width.to_string().into_bytes());
  buf.push(' ' as u8);
  buf.append(&mut height.to_string().into_bytes());
  buf.push('\n' as u8);

  for row in render {
    for pixel in row {
      if *pixel > threshold {
        buf.push('1' as u8);
      } else {
        buf.push('0' as u8);
      }
      buf.push(' ' as u8);
    }
    buf.push('\n' as u8);
  }
  buf
}

/// Renders pixels black or white based on whether they're negative or positive.
pub fn to_pbm_sign(render: &Vec<Vec<f64>>) -> Vec<u8> {
  let height = render.len();
  let width = render.get(0).unwrap_or(&vec![]).len();

  let mut buf = Vec::new();
  buf.append(&mut "P1".as_bytes().into());
  buf.push('\n' as u8);
  buf.append(&mut width.to_string().into_bytes());
  buf.push(' ' as u8);
  buf.append(&mut height.to_string().into_bytes());
  buf.push('\n' as u8);

  for row in render {
    for pixel in row {
      if pixel.is_sign_positive() {
        buf.push('1' as u8);
      } else {
        buf.push('0' as u8);
      }
      buf.push(' ' as u8);
    }
    buf.push('\n' as u8);
  }
  buf
}


/// Renders its input (a 2-array of f64 values) to a PGM file.
/// Assumes values are broadly in the 0.0 : 100.0 range.
pub fn to_pgm(render: &Vec<Vec<f64>>) -> Vec<u8> {
  let height = render.len();
  let width = render.get(0).unwrap_or(&vec![]).len();

  let mut buf = Vec::new();
  buf.append(&mut "P2".as_bytes().into());
  buf.push('\n' as u8);
  buf.append(&mut width.to_string().into_bytes());
  buf.push(' ' as u8);
  buf.append(&mut height.to_string().into_bytes());
  buf.push('\n' as u8);
  buf.append(&mut "65535".to_string().into_bytes());
  buf.push('\n' as u8);

  for row in render {
    for pixel in row {
      let value = (pixel.abs() * 655.0).floor() as u16;
      buf.append(&mut value.to_string().as_bytes().into());
      buf.push(' ' as u8);
    }
    buf.push('\n' as u8);
  }
  buf
}
