// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - cpp.rs
// ===========================================

use std::io::prelude::*;
use std::io::BufReader;
// File
use std::fs::File;

pub mod check_include;
pub mod cat;
pub mod rm_duplication;
pub mod func_list;
pub mod search_keyword;
// grandchild
pub mod structs {
    pub mod reference;
    pub mod start_end_split_string;
}

type Reference = super::cpp::structs::reference::Reference;

pub fn cpp_func(args_: Vec<String>) -> Vec::<Reference>
{
    let mut search_from_path = Vec::<String>::new();
    let mut files = Vec::<String>::new();
    let mut yaml_file_path = String::new();
    let mut include_list = Vec::<String>::new();

    let mut cat_string = String::new();
    let mut programs = Vec::<String>::new();

    let mut args_target_file = Vec::<String>::new();
    for path in &args_[1..] {
        if path.ends_with(".cpp") {
            args_target_file.push(path.to_string());
        }
        else if path.ends_with(".yaml") {
            yaml_file_path = path.to_string();
        }
        else {
            ;
        }
    }

    for path in &args_target_file {
        let entry_point_realpath = std::fs::canonicalize(path).unwrap();
        let entry_point_folder = entry_point_realpath.parent().unwrap().to_str();
        search_from_path.push(entry_point_folder.unwrap().to_string());

        programs.push(path.to_string());
    }
    let yaml_path = yaml_file_path;

    // add args file path
    for path in &programs {
        files.push(std::fs::canonicalize(path).unwrap().to_str().unwrap().to_string());
    }

    for path in &files {
        include_list.append(&mut check_include::check_include(path, &search_from_path));
    }

    // args[1] , include_list --> files
    for path in &include_list {
        files.push(path.to_string());
    }

    let unique_files = rm_duplication::rm_duplication(files);

    cat_string = cat::cat_files(unique_files);

    // first depth keys
    let mut result = Vec::<Reference>::new();

    // open yaml (yaml_path)
    let mut keys = Vec::<String>::new();
    let yaml_path = yaml_path;
    let yaml_file = File::open(yaml_path.clone()).unwrap();
    let yaml_reader = BufReader::new(yaml_file);

    // get keys (fist depth)
    for line in yaml_reader.lines() {
        let line = line.unwrap();
        if line.starts_with("  ") {
        }
        else {
            let key = line.split(":").collect::<Vec<&str>>()[0].to_string();
            keys.push(key);
        }
    }
    keys.retain(|x| x != "");

    for key in &keys {
        let mut _refer = search_keyword::get_reference(yaml_path.clone(), key);
        let mut _result = search_keyword::search_keyword(cat_string.clone(), _refer);
        result.append(&mut _result);
    }

    result
}
