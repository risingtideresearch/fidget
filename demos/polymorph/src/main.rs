use std::fs::File;
use std::io::Write;

use fidget::futhark::cli::*;
use fidget::futhark::netpbm::*;

fn main() {
    let name = "hi"; // hi, prospero, etc. prospero currently requires heavy resources
    // let _ = write_viz("demos/polymorph/prospero").unwrap();
    // let _ = write_pruned_vm("demos/polymorph/prospero").unwrap();
    let _ = write_generated_futhark("demos/polymorph", name, 256, 256).unwrap();
    let stdout = run_rendered_futhark("demos/polymorph", name).unwrap();
    let output = parse_futhark_output_as_2d_array(&stdout);
    let mut image = File::create(format!("demos/polymorph/{name}.ppm")).unwrap();
    let _ = image.write_all(&to_pbm_sign(&output)).unwrap();
}
