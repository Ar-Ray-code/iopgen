// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - reference.rs
// ===========================================
use std::collections::HashMap;
use std::fmt;
pub type StartEndSplitString = super::start_end_split_string::StartEndSplitString;


pub struct Reference {
    pub ref_type: String,
    pub keyword: String,
    pub args: HashMap<String, String>,
    pub args_ab: HashMap<String, StartEndSplitString>,
}

impl Reference {
    pub fn new() -> Reference {
        Reference {
            ref_type: String::new(),
            keyword: String::new(),
            args: HashMap::new(),
            args_ab: HashMap::new(),
        }
    }
}

impl fmt::Debug for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ref_type: {:?}\n" , self.ref_type)?;
        write!(f, "keyword: {:?}\n" , self.keyword)?;
        write!(f, "  - args: {:?}\n" , self.args)?;
        write!(f, "  - args_ab: {:?}\n" , self.args_ab)?;
        Ok(())
    }
}