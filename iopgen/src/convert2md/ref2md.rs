// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - ref2md.rs
// ===========================================

use crate::cpp::structs::reference::Reference;


fn convert_one_header(_refs: &Vec<Reference>, _target_ref_type: &str, _types: &Vec<&str> , _header: String) -> String {
    let mut md_string = String::new();
    md_string.push_str(&_header);
    
    let mut i = 0;
    for ref_ in _refs {
        if ref_.ref_type == _target_ref_type {
            md_string.push_str(&"| ");
            for type_ in _types {
                let _type_str = type_.to_string();
                let arg = ref_.args.get(&_type_str).unwrap();
                md_string.push_str(&arg);
                md_string.push_str(&" | ");
            }
            i += 1;
        }
        if i > 0 {
            md_string.push_str(&"\n");
        }
    }
    md_string
}

pub fn convert(refs: Vec<Reference>) -> String {
    let mut md_string = String::new();

    let sub_header = String::from("# Reference
\n## Subscribe\n
| topic_name | message type | qos_profile | callback |
| --- | --- | --- | --- |\n");

    let sub_types = vec!["topic_name", "msg_type", "qos_profile", "callback"];
    md_string.push_str(&convert_one_header(&refs, "subscribe", &sub_types, sub_header));


    let pub_header = String::from("## Publish\n
| topic_name | message type | qos_profile |
| --- | --- | --- |\n");

    let pub_types = vec!["topic_name", "msg_type", "qos_profile"];
    md_string.push_str(&convert_one_header(&refs, "publish", &pub_types, pub_header));


    let param_header = String::from("## Parameter\n
| parameter_name | default_value |
| --- | --- |\n");

    let param_types = vec!["parameter_name", "default_value"];
    md_string.push_str(&convert_one_header(&refs, "parameter", &param_types, param_header));

    md_string
}


