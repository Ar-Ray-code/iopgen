// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - 😺.rs
// ===========================================
// concatinate files

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// input string vector
pub fn cat_files(path_list: Vec<String>) -> String {
    let mut cat_string = String::new();
    for path in path_list {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            cat_string.push_str(&line);
            cat_string.push_str(&"
");
        }
        cat_string.push_str(&"
");

    }
    cat_string
}