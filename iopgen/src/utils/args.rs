// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - args.rs
// ===========================================


extern crate clap;
use clap::{App, Arg, AppSettings};


// parse_args ============================
// input: args
// output:
// - yaml_path (String)
// - codes (Vec<String>)
// - output_path (String)
// ===========================================
pub fn parse_args() -> (String, Vec<String>, String) {
    // using clap
    let matches = App::new("iopgen")
        .version("0.1.0")
        .author("Ar-Ray-code")
        .about("Generate iop file from cpp codes")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("yaml")
            .short('y')
            .long("yaml")
            .value_name("YAML")
            .help("YAML file path")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("codes")
            .short('c')
            .long("codes")
            .value_name("CODES")
            .help("CPP codes file path")
            .takes_value(true)
            .required(true)
            .multiple(true))
        .arg(Arg::with_name("output")
            .short('o')
            .long("output")
            .value_name("OUTPUT")
            .help("Output file path")
            .takes_value(true)
            .required(true))
        .get_matches();


    let yaml_path = matches.value_of("yaml").unwrap().to_string();
    let codes = matches.values_of("codes").unwrap().map(|s| s.to_string()).collect();
    let output_path = matches.value_of("output").unwrap().to_string();

    // args printf
    println!("============ args ============");
    println!("yaml: {}", yaml_path);
    println!("codes: {:?}", codes);
    println!("output: {}", output_path);
    println!("==============================\n");

    (yaml_path, codes, output_path)
}
