// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - cpp.rs
// ===========================================

pub mod check_include;
pub mod cat;
pub mod rm_duplication;
pub mod search_keyword;

use crate::utils::yaml_reader::{get_yaml_object};
use crate::utils::string_utils::{get_1st_depth_keys};

type Reference = crate::structs::reference::Reference;

// cpp_func ============================
// input:
// - yaml_path (String)
// - codes (Vec<String>)
// output:
// - result (Vec::<Reference>)
// ===========================================

pub fn cpp_func(yaml_path: String, codes: Vec<String>) -> Vec::<Reference>
{
    let mut result = Vec::<Reference>::new();

    let mut programs = Vec::<String>::new();
    let mut search_from_path = Vec::<String>::new();
    let mut files = Vec::<String>::new();
    let mut include_list = Vec::<String>::new();
    let mut _cat_string = String::new();
    
    for path in &codes {
        let entry_point_realpath = std::fs::canonicalize(path).unwrap();
        let entry_point_folder = entry_point_realpath.parent().unwrap().to_str();
        search_from_path.push(entry_point_folder.unwrap().to_string());

        programs.push(path.to_string());
    }

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
    _cat_string = cat::cat_files(unique_files);

    // println!("cat_string: {}", _cat_string);

    let yaml_object = &get_yaml_object(yaml_path);
    let keys = get_1st_depth_keys(yaml_object);
    

    for key in &keys {
        println!("processing: {} ...", key);
        let _refer = search_keyword::get_reference(yaml_object, key);
        // println!("refer: {:?}", _refer);
        let mut _result = search_keyword::search_keyword(_cat_string.clone(), _refer);
        // println!("result: {:?}", _result);

        result.append(&mut _result);
    }

    result
}
