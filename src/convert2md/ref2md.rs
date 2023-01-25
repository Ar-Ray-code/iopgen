// ================ iopgen ===================
// Copyright (c) 2023 Ar-Ray-code
// Licensed under the Apache License, Version 2.0
// 
// - ref2md.rs
// ===========================================

use crate::structs::reference::Reference;


static template_footer: &str  = "
\n<br>\n

## $(0)\n
| $(1) | $(2) | $(3) | $(4) |\n| --- | --- | --- | --- |\n| Ubuntu20 | amd64 | Galactic | |\n\n<br>

## $(5)\n
- $(6)\n\n- $(7)\n\n<br>

## $(8)\n
```bash\n# write here\n```\n\n<br>

## $(9)\n
```bash\n# write here\n```\n\n<br>

$(10)\n\n<br>

## $(11)

$(12)

- [Ar-Ray-code/iopgen](https://github.com/Ar-Ray-code/iopgen)\n\n<br>
";



fn convert_one_header(_refs: &Vec<Reference>, _target_ref_type: &str, _types: &Vec<&str> , _header: String) -> String {
    let mut md_string = String::new();
    md_string.push_str(&_header);

    let mut i = 0;
    let mut i_old = i;
    for ref_ in _refs {
        if ref_.ref_type == _target_ref_type {
            md_string.push_str(&"| ");
            for type_ in _types {                
                let _type_str = type_.to_string();
                let arg = ref_.args.get(&_type_str).unwrap();
                // remove ' " from arg
                let arg = arg.replace("\"", "");
                let arg = arg.replace("'", "");

                md_string.push_str(&"`");
                md_string.push_str(&arg);
                md_string.push_str(&"` | ");
            }
            i += 1;
        }
        if i > i_old {
            md_string.push_str(&"\n");
            i_old = i;
        }
    }
    md_string
}

fn gen_footer(jp_flag: bool) -> String {
    // translation
    let mut _footer = template_footer.clone().to_string();

    let footer_en = vec![
        ("Operation check status", "動作確認状況"), // 0
        ("OS", "OS"), // 1
        ("Arch", "アーキテクチャ"), // 2
        ("Distro", "ディストリビューション"), // 3
        ("status", "状況"), // 4
        ("Requirement", "要件"), // 5 
        ("ROS2", "ROS2"),  // 6
        ("Laptop", "ラップトップ"), //7
        ("How to build", "ビルド方法"), // 8
        ("Run", "実行方法"), // 9
        ("Add a picture of the operation result.", "実行結果の画像を追加する。"), // 10
        ("Reference", "参考"), // 11
        ("Add a reference to the source code.", "ソースコードへの参照を追加する。"), // 12
    ];

    // let list_table_en = vec![
    //     ("Subscribe (Input)", "購読 (入力)"),
    //     ("topic_name", "トピック名"),
    //     ("message type", "メッセージ型"),
    //     ("qos_profile", "QoSプロファイル"),
    //     ("callback", "コールバック"),
    //     ("description", "説明"),
    //     ("Publish (Output)", "公開 (出力)"),
    //     ("Parameter", "パラメータ"),
    //     ("parameter_name", "パラメータ名"),
    //     ("default_value", "デフォルト値"),
    // ];

    let mut i = 0;
    for (en, jp) in footer_en {
        // "$(" + i + ")"
        let _target_number = format!("$({})", i);
        if jp_flag {
            _footer = _footer.replace(&_target_number, jp);
        } else {
            _footer = _footer.replace(&_target_number, en);
        }
        i += 1;
    }
    _footer
}

fn table_split_gen(data_len: i32) -> String {
    // 5 -> "| --- | --- | --- | --- | --- |\n"
    let mut _out = "|".to_string();

    for _ in 0..data_len {
        _out = _out + " --- |";
    }

    _out = _out + "\n";
    _out
}

pub fn convert(refs: Vec<Reference>, title: &str, jp_flag: bool) -> String {
    let mut md_string = String::new();
    let _footer = gen_footer(jp_flag);

    // println!("convert: refs: {:?}", refs);

    if !jp_flag {

        let sub_header = String::from("# ".to_owned() + title +
"\n<br>\n\n## Subscribe (Input)\n
| topic_name | message type | qos_profile | description |\n"
+ table_split_gen(4).as_str());

        let sub_types = vec!["topic_name", "msg_type", "qos_profile"];
        md_string.push_str(&convert_one_header(&refs, "subscribe", &sub_types, sub_header));


        let pub_header = String::from("\n<br>\n\n## Publish (Output)\n
| topic_name | message type | qos_profile | description |\n".to_owned() + table_split_gen(4).as_str());

        let pub_types = vec!["topic_name", "msg_type", "qos_profile"];
        md_string.push_str(&convert_one_header(&refs, "publish", &pub_types, pub_header));


        let param_header = String::from("\n<br>\n\n## Parameter\n\n
| parameter_name | default_value | description |\n".to_owned() + table_split_gen(3).as_str());

        let param_types = vec!["parameter_name", "default_value"];
        md_string.push_str(&convert_one_header(&refs, "parameter", &param_types, param_header));
    }
    else {
        let sub_header = String::from("# ".to_owned() + title +
"\n<br>\n\n## 購読 (入力)\n
| トピック名 | メッセージ型 | QoSプロファイル | 説明 |\n" + table_split_gen(4).as_str());

        let sub_types = vec!["topic_name", "msg_type", "qos_profile"];
        md_string.push_str(&convert_one_header(&refs, "subscribe", &sub_types, sub_header));
// | トピック名 | メッセージ型 | QoSプロファイル | コールバック | 説明 |\n" + table_split_gen(5).as_str());

        // let sub_types = vec!["topic_name", "msg_type", "qos_profile", "callback"];
        // md_string.push_str(&convert_one_header(&refs, "subscribe", &sub_types, sub_header));


        let pub_header = String::from("\n<br>\n\n## 公開 (出力)\n
| トピック名 | メッセージ型 | QoSプロファイル | 説明 |\n".to_owned() + table_split_gen(4).as_str());

        let pub_types = vec!["topic_name", "msg_type", "qos_profile"];
        md_string.push_str(&convert_one_header(&refs, "publish", &pub_types, pub_header));


        let param_header = String::from("\n<br>\n\n## パラメータ\n\n
| パラメータ名 | デフォルト値 | 説明 |\n".to_owned()+ table_split_gen(3).as_str());

        let param_types = vec!["parameter_name", "default_value"];
        md_string.push_str(&convert_one_header(&refs, "parameter", &param_types, param_header));

    }

    md_string.push_str(&_footer);

    md_string
}
