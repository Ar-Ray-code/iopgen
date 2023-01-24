// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - main.rs
// ===========================================

use std::env;
use std::io::prelude::*;

mod cpp;
mod convert2md;

type Reference = cpp::structs::reference::Reference;

fn main() {
    let args: Vec<String> = env::args().collect();
    
   
    if args.len() < 3 {
        println!("Usage: iopgen [file] [export to md]");
        return;
    }

    let mut refs = Vec::<Reference>::new();

    if args[1].clone().ends_with(".cpp") {
        refs = cpp::cpp_func(args.clone());
    }

    let mut target_md = String::new();
    for path in &args[1..] {
        if path.ends_with(".md") {
            target_md = path.to_string();
        }
    }
    
    let md_data = convert2md::ref2md::convert(refs);
    // write
    let mut file = std::fs::File::create(target_md).unwrap();
    file.write_all(md_data.as_bytes()).unwrap();

}
