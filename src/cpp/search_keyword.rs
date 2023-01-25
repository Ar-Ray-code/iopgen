// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - search_keyword.rs
// ===========================================

extern crate yaml_rust;
type Reference = crate::structs::reference::Reference;

use crate::utils::string_utils::{get_arg, get_type};

// get_reference ============================
// input:
//   - yaml_object: yaml object (Yaml)
//   - target_key: target key
// output:
//   - Reference
// ===========================================

pub fn get_reference(yaml_object: &yaml_rust::Yaml, target_key: &str) -> Reference {
    let mut reference = Reference::new();

    reference.keyword = yaml_object[target_key]["keyword"].as_str().unwrap().to_string();
    reference.ref_type = target_key.to_string();

    // let mut i = 0;
    for arg in yaml_object[target_key]["args"].as_vec().unwrap() {
        let arg_name = arg.as_hash().unwrap().keys().collect::<Vec<&yaml_rust::Yaml>>()[0].as_str().unwrap().to_string();
        let mode = arg[arg_name.as_str()].as_i64().unwrap();

        reference.args_ab.insert(arg_name.clone(), mode);
        reference.args.insert(arg_name.clone(), arg_name);
        // i += 1;
    }
    // println!("reference: {:?}", reference);
    reference
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
