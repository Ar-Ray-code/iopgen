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
// grandchild
pub mod structs {
    pub mod reference;
    pub mod start_end_split_string;
}

pub fn cpp_func(args_: Vec<String>)
{
    let mut search_from_path = Vec::<String>::new();
    let mut files = Vec::<String>::new();
    let mut yaml_file_path = String::new();
    let mut include_list = Vec::<String>::new();

    let mut cat_string = String::new();
    let mut programs = Vec::<String>::new();

    for path in &args_[1..] {
        let entry_point_realpath = std::fs::canonicalize(path).unwrap();
        let entry_point_folder = entry_point_realpath.parent().unwrap().to_str();
        search_from_path.push(entry_point_folder.unwrap().to_string());
        // if .yaml
        if path.ends_with(".yaml") {
            yaml_file_path = path.to_string();
        }
        else {
            programs.push(path.to_string());
        }
    }
    let yaml_path = yaml_file_path;
    // println!("{:?}", yaml_path);

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
    // println!("{:?}", unique_files);
 
    cat_string = cat::cat_files(unique_files);

    let pub_refer = search_keyword::get_reference(yaml_path.clone(), "pub");
    let sub_refer = search_keyword::get_reference(yaml_path.clone(), "sub");
    let param_refer = search_keyword::get_reference(yaml_path.clone(), "param");
    println!("{:?}", pub_refer);
    println!("{:?}", sub_refer);
    println!("{:?}", param_refer);

    let result_pub = search_keyword::search_keyword(cat_string.clone(), pub_refer);
    println!("{:?}", result_pub);
    let result_sub = search_keyword::search_keyword(cat_string.clone(), sub_refer);
    println!("{:?}", result_sub);
    let result_param = search_keyword::search_keyword(cat_string.clone(), param_refer);
    // println!("-------- summary --------");
    
    // println!("{:?}", result_sub);
    println!("{:?}", result_param);
    // _topic_name = split_by_ab(output_line.clone());
        // _msg_type = split_by_ab(output_line.clone(), "<".to_string(), ">".to_string());
        // _qos_profile = split_by_ab(output_line.clone(), "\",".to_string(), ");".to_string());
    // let result = search_keyword::search_keyword(cat_string, pub_refer);
    // println!("{:?}", result);

}
