// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - main.rs
// ===========================================

use std::env;

mod cpp;


fn main() {
    let args: Vec<String> = env::args().collect();
    
   
    if args.len() < 2 {
        println!("Usage: iopgen [file]");
        return;
    }

    if args[1].ends_with(".cpp") {
        cpp::cpp_func(args);
    }

}
