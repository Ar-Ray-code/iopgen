// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - search_keyword.rs
// ===========================================
// search keywords from source code (from yaml file)

use std::fs::File;
use std::io::prelude::*;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

pub type StartEndSplitString = super::structs::start_end_split_string::StartEndSplitString;
pub type Reference = super::structs::reference::Reference;


// get_reference ============================
pub fn get_reference(yaml_path: String, target_key: &str) -> super::structs::reference::Reference {
    let mut reference = Reference::new();

    // open yaml file
    let mut file = File::open(yaml_path).unwrap();
    let mut yaml_docs = String::new();

    file.read_to_string(&mut yaml_docs).unwrap();

    let yaml_src = YamlLoader::load_from_str(&yaml_docs).unwrap();
    let doc = &yaml_src[0];

    reference.keyword = doc[target_key]["keyword"].as_str().unwrap().to_string();
    reference.ref_type = target_key.to_string();


    for arg in doc[target_key]["args"].as_vec().unwrap() {
        // let arg = arg.as_str().unwrap();
        let arg_name = arg.as_hash().unwrap().keys().collect::<Vec<&yaml_rust::Yaml>>()[0].as_str().unwrap().to_string();

        let arg_start = arg["start"].as_str().unwrap().to_string();
        let arg_end = arg["end"].as_str().unwrap().to_string();
        let mut arg_ab = StartEndSplitString::new();
        arg_ab.start = arg_start;
        arg_ab.end = arg_end;
        reference.args_ab.insert(arg_name.clone(), arg_ab);
        reference.args.insert(arg_name.clone(), arg_name);
    }
    reference
}


fn split_by_ab(src: String, a: String, b: String) -> String {
    let mut output = src.clone();
    output = output.split(&a).collect::<Vec<&str>>()[1].to_string();
    output = output.split(&b).collect::<Vec<&str>>()[0].to_string();
    output
}

pub fn search_keyword(src: String, refer: super::structs::reference::Reference) -> Vec<super::structs::reference::Reference> {
    let mut refs = Vec::new();
    let lines = src.split("\n");

    // find pub name -----------------------------------
    let mut tmp_lines = lines.clone();
    let mut i = 0;
    let mut find_pub_line = Vec::<String>::new();
    let mut tmp_pub_line = String::new();
    let mut line_push_flag = false;

    for line in &mut tmp_lines {
        if line.contains(&refer.keyword) || line_push_flag {
            if !line_push_flag {
                line_push_flag = true;
                tmp_pub_line = String::new();
            }
            else {
                if line.contains(";") {
                    line_push_flag = false;

                    let mut output_line = tmp_pub_line.clone() + line;
                    output_line = output_line.replace(" ", "");
                    find_pub_line.push(output_line.clone());

                    let mut _topic_name = String::new();
                    let mut _msg_type = String::new();
                    let mut _qos_profile = String::new();
                    let mut ref_ = Reference::new();


                    // switch using ref_type
                    if refer.ref_type == "pub" {
                        _topic_name = split_by_ab(output_line.clone(), refer.args_ab["topic_name"].start.clone(), refer.args_ab["topic_name"].end.clone());
                        _msg_type = split_by_ab(output_line.clone(), refer.args_ab["msg_type"].start.clone(), refer.args_ab["msg_type"].end.clone());
                        _qos_profile = split_by_ab(output_line.clone(), refer.args_ab["qos_profile"].start.clone(), refer.args_ab["qos_profile"].end.clone());

                        ref_.args.insert("topic_name".to_string(), _topic_name.clone());
                        ref_.args.insert("msg_type".to_string(), _msg_type.clone());
                        ref_.args.insert("qos_profile".to_string(), _qos_profile.clone());
                        refs.push(ref_);
                    }
                    else if refer.ref_type == "sub" {
                        _topic_name = split_by_ab(output_line.clone(), refer.args_ab["topic_name"].start.clone(), refer.args_ab["topic_name"].end.clone());
                        _msg_type = split_by_ab(output_line.clone(), refer.args_ab["msg_type"].start.clone(), refer.args_ab["msg_type"].end.clone());
                        _qos_profile = split_by_ab(output_line.clone(), refer.args_ab["qos_profile"].start.clone(), refer.args_ab["qos_profile"].end.clone());

                        ref_.args.insert("topic_name".to_string(), _topic_name.clone());
                        ref_.args.insert("msg_type".to_string(), _msg_type.clone());
                        ref_.args.insert("qos_profile".to_string(), _qos_profile.clone());
                        refs.push(ref_);
                    }
                    else if refer.ref_type == "param" {
                        let mut _parameter_name = String::new();
                        let mut _default_value = String::new();

                        _parameter_name = split_by_ab(output_line.clone(), refer.args_ab["parameter_name"].start.clone(), refer.args_ab["parameter_name"].end.clone());
                        _default_value = split_by_ab(output_line.clone(), refer.args_ab["default_value"].start.clone(), refer.args_ab["default_value"].end.clone());

                        ref_.args.insert("parameter_name".to_string(), _parameter_name.clone());
                        ref_.args.insert("default_value".to_string(), _default_value.clone());
                        refs.push(ref_);
                    }
                }
            }
            tmp_pub_line = tmp_pub_line + line;
        }
        i += 1;
    }
    refs
}
