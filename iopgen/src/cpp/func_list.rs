// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - func_list.rs
// ===========================================

// generate punc_list from string
// How to
// search `{` hierarchy in string (1st depth)
// search `}` hierarchy in string (1st depth)


pub fn func_list(src: &str) -> Vec<String> {

    let mut func_list: Vec<String> = Vec::new();
    
    let mut depth: i32 = 0;

    let mut class_flag: bool = false;
    let mut struct_flag: bool = false;
    let mut namespace_flag: bool = false;
    let mut enum_flag: bool = false;
    
    let mut cache: String = String::new();
    for c in src.chars() {
        if c == '{' {
            println!("update depth!(: {}", depth);
            depth += 1;
            // // add cache to func_name
            if depth == 1 {
                let split_list: Vec<&str> = cache.split('\n').collect();
                let last_word: String = split_list[split_list.len() - 2].to_string();
                // class? struct? namespace? enum?
                if last_word == "class" {
                    class_flag = true;
                }
                else if last_word == "struct" {
                    struct_flag = true;
                }
                else if last_word == "namespace" {
                    namespace_flag = true;
                }
                else if last_word == "enum" {
                    enum_flag = true;
                }
                else {
                    func_list.push(last_word);
                }
                // println!("last_word: {}", last_word);
            }
            else if depth > 1 {
                if class_flag {
                    // class
                    if c == '}' {
                        class_flag = false;
                    }
                }
                else if struct_flag {
                    // struct
                    if c == '}' {
                        struct_flag = false;
                    }
                }
                else if namespace_flag {
                    // namespace
                    if c == '}' {
                        namespace_flag = false;
                    }
                }
                else if enum_flag {
                    // enum
                    if c == '}' {
                        enum_flag = false;
                    }
                }
                else {
                    // function
                }
            }
            // // cache clear
            cache.clear();
        }
        if c == '}' {
            depth -= 1;
            cache.clear();
        }
        else if depth == 0 {
            // add cache
            cache.push(c);
        }
    }
    func_list
}
