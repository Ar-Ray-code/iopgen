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

type StartEndSplitString = super::structs::start_end_split_string::StartEndSplitString;
type Reference = super::structs::reference::Reference;


// get_reference ============================
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
        let mut arg_ab = StartEndSplitString::new();

        arg_ab.start = arg["start"].as_str().unwrap().to_string();
        arg_ab.end = arg["end"].as_str().unwrap().to_string();

        reference.args_ab.insert(arg_name.clone(), arg_ab);
        reference.args.insert(arg_name.clone(), arg_name);
        i += 1;
    }
    reference
}


fn split_by_ab(src: String, a: String, b: String) -> String {
    let mut output = src.clone();
    output = output.split(&a).collect::<Vec<&str>>()[1].to_string();
    output = output.split(&b).collect::<Vec<&str>>()[0].to_string();
    output
}

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
                        let arg_start = refer.args_ab[arg].start.clone();
                        let arg_end = refer.args_ab[arg].end.clone();
                        let arg_value = split_by_ab(output_line.clone(), arg_start, arg_end);

                        // println!("{}:" , arg.clone());
                        // println!("- front: {}", refer.args_ab[arg].start.clone());
                        // println!("- back: {}", refer.args_ab[arg].end.clone());
                        // println!("- value: {}", arg_value.clone());

                        ref_.line = i as i32;
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
