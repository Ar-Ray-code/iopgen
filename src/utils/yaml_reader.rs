// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - yaml_reader.rs
// ===========================================

use std::fs::File;
use std::io::prelude::*;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

// YAML Reader ============================
pub fn get_yaml_raw(yaml_path: String) -> String {
    let mut yaml_file = File::open(yaml_path).unwrap();
    let mut yaml_src = String::new();

    yaml_file.read_to_string(&mut yaml_src).unwrap();
    yaml_src
}

pub fn get_yaml_object(yaml_path: String) -> yaml_rust::Yaml {
    let yaml_src = get_yaml_raw(yaml_path);
    let yaml_object = YamlLoader::load_from_str(&yaml_src).unwrap();
    yaml_object[0].clone()
}
