// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - main.rs
// ===========================================
use std::io::prelude::*;

mod cpp;
mod convert2md;
mod utils;
mod structs;

fn main() {
    let mut output_data = String::new();
    
    let (yaml_path, codes, output_path, title, jpn) =
        utils::args::parse_args();

    let refs = cpp::cpp_func(yaml_path, codes);
    // println!("refs: {:?}", refs);
    
    if output_path.ends_with(".md") {
        output_data = convert2md::ref2md::convert(refs, title.as_str(), jpn);
    }

    // write to file
    let mut file = std::fs::File::create(output_path).unwrap();
    file.write_all(output_data.as_bytes()).unwrap();

}
