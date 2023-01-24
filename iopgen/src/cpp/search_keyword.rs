// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - search_keyword.rs
// ===========================================

use std::fs::File;
use std::io::prelude::*;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

type Reference = super::structs::reference::Reference;


// get_reference ============================
// input:
//   - yaml_path: yaml file path
//   - target_key: target key
// output:
//   - Reference

pub fn get_reference(yaml_path: String, target_key: &str) -> Reference {
    let mut reference = Reference::new();

    // open yaml file -------------------------
    let mut file = File::open(yaml_path).unwrap();
    let mut yaml_docs = String::new();

    file.read_to_string(&mut yaml_docs).unwrap();

    let yaml_src = YamlLoader::load_from_str(&yaml_docs).unwrap();
    let doc = &yaml_src[0];
    // ----------------------------------------

    reference.keyword = doc[target_key]["keyword"].as_str().unwrap().to_string();
    reference.ref_type = target_key.to_string();

    let mut i = 0;
    for arg in doc[target_key]["args"].as_vec().unwrap() {
        let arg_name = arg.as_hash().unwrap().keys().collect::<Vec<&yaml_rust::Yaml>>()[0].as_str().unwrap().to_string();
        // let mut arg_ab = StartEndSplitString::new();
        // publish:
        // keyword: create_publisher
        // args:
        //   - topic_name: 1
        //   - msg_type: -1
        //   - qos_profile: 2

        let mode = arg[arg_name.as_str()].as_i64().unwrap();

        reference.args_ab.insert(arg_name.clone(), mode);
        reference.args.insert(arg_name.clone(), arg_name);
        i += 1;
    }
    // println!("reference: {:?}", reference);
    reference
}

// split_by_ab ============================
// - input:
//   - "create_publisher(topic_name, msg_type, qos_profile)"
//   - "("
//   - ")"
// - output: "topic_name, msg_type, qos_profile"

fn split_by_ab(src: String, a: String, b: String) -> String {
    let mut output = src.clone();
    output = output.split(&a).collect::<Vec<&str>>()[1].to_string();
    output = output.split(&b).collect::<Vec<&str>>()[0].to_string();
    output
}

// get_arg ============================
// - input:
//   - "create_publisher(topic_name, msg_type, qos_profile)"
//   - 0
// - output: "topic_name"

fn get_arg(src: String, position: i64) -> String {
    let mut output = src.clone();
    output = split_by_ab(output, "(".to_string(), ");".to_string());
    let args = output.split(",").collect::<Vec<&str>>();
    // println!("args: {:?}", args);
    args[position as usize].to_string()
}

// get_type ============================
// - input: "create_publisher(topic_name, msg_type, qos_profile)"
// - output: "msg_type"

fn get_type(src: String) -> String {
    let mut output = src.clone();
    // get < to >
    output = split_by_ab(output, "<".to_string(), ">".to_string());
    output
}

// search_keyword ============================
// input:
//   - src: "create_publisher(topic_name, msg_type, qos_profile);"
//   - refer: Reference
// output:
//   - vec![Reference]

pub fn search_keyword(src: String, refer: Reference) -> Vec<Reference> {
    let mut refs = Vec::new();
    let lines = src.split("\n");

    // find pub name -----------------------------------
    let mut tmp_lines = lines.clone();
    let mut i = 0;
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

                    let mut ref_ = Reference::new();
                    ref_.keyword = refer.keyword.clone();
                    ref_.ref_type = refer.ref_type.clone();

                    let mut output_line = tmp_pub_line.clone() + line;

                    output_line = output_line.replace(" ", "");

                    // println!("------------------------------------");
                    // println!("refer.args_ab.keys(): {:?}", refer.args_ab.keys());
                    // println!("args_ab: {:?}", refer.args_ab);
                    // println!("------------------------------------");

                    for arg in refer.args_ab.keys() {
                        // args_ab[topic_name] = 1 --> topic_name
                        // args_ab[msg_type] = -1 --> msg_type
                        // args_ab[qos_profile] = 2 --> qos_profile
                        if refer.args_ab[arg] < 0 {
                            ref_.args.insert(arg.clone(), get_type(output_line.clone()));
                            continue;
                        }
                        let arg_value = get_arg(output_line.clone(), refer.args_ab[arg]);

                        ref_.line = i as i64;
                        ref_.args.insert(arg.clone(), arg_value.clone());
                    }
                    refs.push(ref_);
                }
            }
            tmp_pub_line = tmp_pub_line + line;
        }
        i += 1;
    }
    refs
}
