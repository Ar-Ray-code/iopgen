// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - search_keyword.rs
// ===========================================
// search keywords from source code (from yaml file)

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

extern crate yaml_rust;

use yaml_rust::{YamlLoader, YamlEmitter};

pub struct Reference {
    pub pub_keyword: String,
    pub pub_args: HashMap<String, String>,
    pub sub_keyword: String,
    pub sub_args: HashMap<String, String>,
    pub param_keyword: String,
    pub param_args: HashMap<String, String>,
}

impl Reference {
    pub fn new() -> Reference {
        Reference {
            pub_keyword: String::new(),
            pub_args: HashMap::new(),
            sub_keyword: String::new(),
            sub_args: HashMap::new(),
            param_keyword: String::new(),
            param_args: HashMap::new(),
        }
    }
}

impl fmt::Debug for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pub_keyword: {:?}" , self.pub_keyword)?;
        write!(f, "pub_args: {:?}" , self.pub_args)?;
        write!(f, "sub_keyword: {:?}" , self.sub_keyword)?;
        write!(f, "sub_args: {:?}" , self.sub_args)?;
        write!(f, "param_keyword: {:?}" , self.param_keyword)?;
        write!(f, "param_args: {:?}" , self.param_args)?;
        Ok(())
    }
}

pub fn get_reference(yaml_path: String) -> Reference {
    let mut reference = Reference::new();

    // open yaml file
    let mut file = File::open(yaml_path).unwrap();
    let mut yaml_docs = String::new();

    file.read_to_string(&mut yaml_docs).unwrap();

    let yaml_src = YamlLoader::load_from_str(&yaml_docs).unwrap();
    let doc = &yaml_src[0];

    // about pub keyword
    let pub_keyword = doc["pub"]["keyword"].as_str().unwrap();
    reference.pub_keyword = pub_keyword.to_string();
    for arg in doc["pub"]["args"].as_vec().unwrap() {
        let arg = arg.as_str().unwrap();
        reference.pub_args.insert(arg.to_string(), arg.to_string());
    }

    // about sub keyword
    let sub_keyword = doc["sub"]["keyword"].as_str().unwrap();
    reference.sub_keyword = sub_keyword.to_string();
    for arg in doc["sub"]["args"].as_vec().unwrap() {
        let arg = arg.as_str().unwrap();
        reference.sub_args.insert(arg.to_string(), arg.to_string());
    }

    // about param keyword
    let param_keyword = doc["param"]["keyword"].as_str().unwrap();
    reference.param_keyword = param_keyword.to_string();
    for arg in doc["param"]["args"].as_vec().unwrap() {
        let arg = arg.as_str().unwrap();
        reference.param_args.insert(arg.to_string(), arg.to_string());
    }

    reference
}

pub fn search_keyword(src: String, refer: Reference) -> Vec<Reference> {
    let mut refs = Vec::new();

    // println!("src: {:?}", src);

    // split using \n
    let mut lines = src.split("\n");

    // search keyword
    // pub
    for line in &mut lines {
        if line.contains(&refer.pub_keyword) {
            let mut ref_ = Reference::new();
            ref_.pub_keyword = refer.pub_keyword.clone();
            ref_.pub_args = refer.pub_args.clone();
            refs.push(ref_);
        }
    }

    // sub
    for line in &mut lines {
        if line.contains(&refer.sub_keyword) {
            let mut ref_ = Reference::new();
            ref_.sub_keyword = refer.sub_keyword.clone();
            ref_.sub_args = refer.sub_args.clone();
            refs.push(ref_);
        }
    }

    // param
    for line in &mut lines {
        if line.contains(&refer.param_keyword) {
            let mut ref_ = Reference::new();
            ref_.param_keyword = refer.param_keyword.clone();
            ref_.param_args = refer.param_args.clone();
            refs.push(ref_);
        }
    }

    // println!("refs: {:?}", refs);



    refs
}
