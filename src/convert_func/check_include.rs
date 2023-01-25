// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - check_include.rs
// ===========================================

// Open C++ codes and listing include path
// return include path list

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn check_include(path: &str, search_from: &Vec<String>) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut include_list: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("#include") {
            if line.contains("<") {
                continue;
            }
            let mut include_path = line.split(" ").collect::<Vec<&str>>();
            include_path.remove(0);

            let include_path = include_path.join(" ");
            let include_path = include_path.replace("\"", "");

            for search_path in search_from {
                let search_path = search_path.to_string() + "/" + &include_path;
                if std::path::Path::new(&search_path).exists() {
                    include_list.push(search_path);
                    break;
                }
            }
        }
    }
    include_list
}