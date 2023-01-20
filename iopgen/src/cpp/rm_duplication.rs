// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - rm_duplication.rs
// ===========================================

// remove duplication (e.g. abc.h and abc.h) from string vector

pub fn rm_duplication(path_list: Vec<String>) -> Vec<String> {
    let mut path_list = path_list;
    path_list.sort();
    path_list.dedup();
    path_list
}
