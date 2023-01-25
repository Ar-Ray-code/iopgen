// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - string_utils.rs
// ===========================================

// split_by_ab ============================
// - input:
//   - "create_publisher(topic_name, msg_type, qos_profile)"
//   - "("
//   - ")"
// - output: "topic_name, msg_type, qos_profile"

pub fn split_by_ab(src: String, a: String, b: String) -> String {
    let mut output = src.clone();
    output = output.split(&a).collect::<Vec<&str>>()[1].to_string();
    output = output.split(&b).collect::<Vec<&str>>()[0].to_string();
    output
}

// get_arg ==================================
// - input:
//   - "create_publisher(topic_name, msg_type, qos_profile)"
//   - 0
// - output: "topic_name"
// ===========================================

pub fn get_arg(src: String, position: i64) -> String {
    let mut output = src.clone();
    output = split_by_ab(output, "(".to_string(), ");".to_string());
    let args = output.split(",").collect::<Vec<&str>>();
    
    args[position as usize].to_string()
    // println!("args: {:?}", args);
}

// get_type ============================
// - input: "create_publisher(topic_name, msg_type, qos_profile)"
// - output: "msg_type"
// ===========================================

pub fn get_type(src: String) -> String {
    let mut output = src.clone();
    // get < to >
    output = split_by_ab(output, "<".to_string(), ">".to_string());
    output
}

// get_1st_depth_keys ============================
// - input: yaml object
// - output: keys
// ===========================================

pub fn get_1st_depth_keys(yaml_object: &yaml_rust::Yaml) -> Vec<String> {
    let mut keys = Vec::new();
    for key in yaml_object.as_hash().unwrap().keys() {
        keys.push(key.as_str().unwrap().to_string());
    }
    keys
}