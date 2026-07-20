use std::{ffi::OsStr, fs::File, io::BufReader, path::PathBuf};

pub fn derive_output(input: &PathBuf) -> PathBuf {
    let mut out = input.clone();

    if let Some(stem) = input.file_stem() {
        out.set_file_name(format!("{}_out", stem.to_string_lossy()));
    } else {
        println!("Warning: The input file has no name, so you should specify an output folder.");
        println!("         Will output to the \"./unscratch_output\" folder instead.");
        out.set_file_name("unscratch_output");
    }

    out
}
