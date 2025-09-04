use crate::context::*;

use std::fs::File;
use std::io::Write;
use std::process::Command;

// discount anyhow
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Write a .vm file to a .viz file, for visualization of node dependencies.
pub fn write_viz(name: &'static str) -> Result<()> {
  let file = std::fs::File::open(format!("{}.vm", name))?;
  let (ctx, node) = Context::from_text(file)?;

  let mut output = File::create(format!("{}.viz", name))?;
  let _ = File::write(&mut output, &ctx.dot().into_bytes())?;
  Ok(())
}

/// Prune a .vm file by converting to/from a Context and a Tree.
pub fn write_pruned_vm(name: &'static str) -> Result<()> {
  let file = File::open(format!("{}.vm", name))?;
  let (ctx, node) = Context::from_text(file)?;

  let (dedup_ctx, dedup_node) = ctx.prune(node)?;
  let mut output = File::create(format!("{}.vm2", name))?;
  let _ = File::write(&mut output, dedup_ctx.to_text().as_bytes())?;
  Ok(())
}

/// Generate and write specialized Futhark for a particular .vm to a file.
pub fn write_generated_futhark(dir: &'static str, name: &'static str, x: usize, y: usize) -> Result<()> {
  let file = File::open(format!("{}/{}.vm", dir, name)).expect("failed to find .vm file");
  let (ctx, node) = Context::from_text(file)?;

  let prelude_futhark = crate::futhark::eval::naive_render;
  // std::fs::read_to_string(format!("{}/num-target.fut", dir)).expect("failed to find prelude file");

  let mut output_futhark = File::create(format!("{dir}/{name}.fut")).expect("failed to create output file");
  let _ = File::write(&mut output_futhark, prelude_futhark.as_bytes())?;
  let _ = File::write(&mut output_futhark, ctx.futhark_inline(node, name.to_owned()).as_bytes())?;
  let _ = File::write(&mut output_futhark, format!("\n\ndef main = render({x}, {y}, {name})\n").as_bytes())?;
  Ok(())
}

/// Run a rendered Futhark file to a value.
/// This just calls into subprocesses, for now.
pub fn run_rendered_futhark(dir: &'static str, name: &'static str) -> Result<String> {
  let output = Command::new("futhark").args(["c", &format!("{dir}/{name}.fut"), "-o", &format!("{dir}/{name}")])
    .output().expect("failed to compile futhark code");
  let render = Command::new(format!("{dir}/{name}"))
    .output().expect("failed to evaluate futhark code");
  Ok(String::from_utf8(render.stdout)?)
}

/// Parse Futhark output into a 2-dimensional array of f64s, specifically.
/// TODO: this is a brittle function and doesn't make an attempt at error reporting / parsing general Futhark output
pub fn parse_futhark_output_as_2d_array(input: &str) -> Vec<Vec<f64>> {
  input.replace("[", "").replace("]]", "]").replace("]\n", "")
       .split("], ")
       .map(|vec|
          vec.split(", ")
             .map(|x| x.strip_suffix("f64").unwrap().parse::<f64>().unwrap())
             .collect::<Vec<_>>()
        ).collect::<Vec<_>>()
}
