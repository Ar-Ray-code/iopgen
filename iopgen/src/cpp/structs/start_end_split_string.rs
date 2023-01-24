// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - reference.rs
// ===========================================
use std::fmt;

// StartEndSplitString ======================
pub struct StartEndSplitString {
    pub start: String,
    pub end: String,
}

impl StartEndSplitString {
    pub fn new() -> StartEndSplitString {
        StartEndSplitString {
            start: String::new(),
            end: String::new(),
        }
    }
}

impl fmt::Debug for StartEndSplitString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "start: {:?}, stop: {:?}\n" , self.start, self.end)?;
        Ok(())
    }
}