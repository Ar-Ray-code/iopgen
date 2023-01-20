// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - cpp.rs
// ===========================================

pub mod check_include;
pub mod cat;
pub mod rm_duplication;
pub mod func_list;
pub mod search_keyword;



pub fn cpp_func(args_: Vec<String>)
{
    let mut search_from_path = Vec::<String>::new();
    let mut files = Vec::<String>::new();

    for path in &args_[1..] {
        let entry_point_realpath = std::fs::canonicalize(path).unwrap();
        let entry_point_folder = entry_point_realpath.parent().unwrap().to_str();
        search_from_path.push(entry_point_folder.unwrap().to_string());
    }

    // let yaml_path = String::from("ros2/ros2.yaml");
    // realpath
    let yaml_path = String::from("/home/ubuntu/Documents/github/iopgen/iopgen/example/ros2/ros2.yaml");

    // add args file path
    for path in &args_[1..] {
        files.push(std::fs::canonicalize(path).unwrap().to_str().unwrap().to_string());
    }

    let mut include_list = Vec::<String>::new();
    for path in &files {
        include_list.append(&mut check_include::check_include(path, &search_from_path));
    }

    // args[1] , include_list --> files
    for path in &include_list {
        files.push(path.to_string());
    }
    let unique_files = rm_duplication::rm_duplication(files);
    println!("{:?}", unique_files);
 
    let mut cat_string = String::new();

    cat_string = cat::cat_files(unique_files);
    // println!("{}", cat_string);

    // let mut func_list = Vec::<String>::new();
    // func_list = func_list::func_list(&cat_string);
    // let unique_func_list = rm_duplication::rm_duplication(func_list);
    // println!("{:?}", unique_func_list);

    // 
    let mut refer = search_keyword::Reference::new();
    // print yaml_path
    println!("{}", yaml_path);
    refer = search_keyword::get_reference(yaml_path);
    // println!("{:?}", refer);
    let result = search_keyword::search_keyword(cat_string, refer);
    println!("{:?}", result);

}
