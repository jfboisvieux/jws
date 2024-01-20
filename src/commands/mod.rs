use crate::input_output::data_file_to_hash;
use crate::utils;
use crate::utils::decrypt;
extern crate magic_crypt;
use regex::Regex;
use serde_json::json;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn display_sorted_data(file_path: PathBuf) {
    let data = data_file_to_hash(file_path);
    let mut key_value: Vec<_> = data.iter().collect();
    key_value.sort_by_key(|a| a.0);

    println!("{}", "\n");
    println!("{: <30}|{: <30}", "Id", "Secret Key");
    println!("{}", "\n");

    for (key, value) in key_value.iter() {
        println!("{: <30}|{: <30}", key, value);
    }
    println!("{}", "\n");
}

// pub fn delete_record(key: &Delete, file_path: PathBuf) {
//     match key.key {
//         Some(ref pattern) => {
//             let matched_records: Vec<(String, String)> =
//                 get_keys_matching_pattern(pattern, &file_path);

//             if matched_records.len() == 0 {
//                 println!("no match");
//             }

//             if matched_records.len() == 1 {
//                 let key_that_match = &matched_records[0].0;
//                 println!("\n \n {} login has been deleted \n\n", key_that_match);
//                 let _ = delete_line_of_record_with_key(key_that_match, &file_path);
//             } else {
//                 let mut counter = 0;
//                 let mut matched_keys: Vec<&String> = Vec::new();
//                 for (key, _password) in matched_records.iter() {
//                     matched_keys.push(key);

//                     counter += 1;
//                     println!("     {: <30}  ->      {: <30} \n ", counter, key);
//                     println!("matched_keys : {:?}", matched_keys);
//                 }

//                 println!("Please select the number that fits your choice");
//                 println!("Or print number 0 to quit delete");
//                 let choice = get_choice();
//                 println!("after call get_choice : choice is {}", choice);

//                 if choice == "0" {
//                     println!("choice is {}", choice);
//                     println!("I return");
//                     return;
//                 } else {
//                     let mut index: usize = choice.parse().unwrap();
//                     index = index - 1;
//                     println!("in else index : {index}");
//                     println!("matched_records : {:?}", matched_keys);
//                     let pattern = matched_keys[index];
//                     println!("Pattern = {pattern}");
//                     let _ = delete_line_of_record_with_key(pattern, &file_path);
//                 }
//             }
//         }
//         None => {
//             println!("\n there is no login that match your input");
//             // println!("example: $ {} [name]\n", short_program_name);
//         }
//     }
// }

pub fn find_by_login(pattern: &str, file_path: PathBuf) -> String {
    let matched_records: Vec<(String, String)> = get_keys_matching_pattern(pattern, &file_path);
    let response = String::new();
    if matched_records.len() == 0 {
        //println!("pattern not found");
        let response = "pattern not found".to_owned();
        response
    } else {
        let json_data: Vec<_> = matched_records
            .iter()
            .map(|(key, password)| json!({ "key": key, "password": password }))
            .collect();

        let json_string = serde_json::to_string_pretty(&json_data).unwrap();
        json_string
    }
}

// pub fn edit_password(editkey: &Edit, file_path: PathBuf) {
//     let key = &editkey.newpass;
//     match key {
//         Some(ref pattern) => {
//             let matched_records: Vec<(String, String)> =
//                 get_keys_matching_pattern(pattern, &file_path);

//             if matched_records.len() == 0 {
//                 println!("no match");
//             }

//             if matched_records.len() == 1 {
//                 //  let key_that_match = &matched_records[0].0;
//                 let _ = change_password_of_record_with_key(pattern, file_path);
//             } else {
//                 let mut counter = 0;
//                 let mut matched_keys: Vec<&String> = Vec::new();
//                 for (key, _) in matched_records.iter() {
//                     matched_keys.push(key);

//                     counter += 1;
//                     println!("\n\n");
//                     println!("     {: <30}  ->      {: <30} \n ", counter, key);
//                     // println!("matched_keys : {:?}", matched_keys);
//                 }

//                 println!("Please select the number that fits your choice");
//                 println!("Or print number 0 to quit delete");
//                 let choice = get_choice();
//                 if choice == "0" {
//                     println!("choice is {}", choice);
//                     println!("I return");
//                     return;
//                 } else {
//                     let mut index: usize = choice.parse().unwrap();
//                     index = index - 1;
//                     println!("in else index : {index}");
//                     println!("matched_records : {:?}", matched_keys);
//                     let pattern = matched_keys[index];
//                     println!("Pattern = {pattern}");
//                     let _ = change_password_of_record_with_key(pattern, file_path);
//                 }
//             }
//         }
//         None => {
//             println!("\n there is no login that match your input");
//             // println!("example: $ {} [name]\n", short_program_name);
//         }
//     }
// }

// fn change_password_of_record_with_key(key: &String, file_path: PathBuf) -> Result<(), io::Error> {
//     println!("Debug in change_password");

//     let new_value_of_password: &str = key;
//     println!("in delete_line_of_record_with_key");
//     let file = File::open(file_path.clone())?;
//     let reader = BufReader::new(&file);
//     let re = Regex::new(new_value_of_password).expect("Invalid regex pattern");

//     let temp_file_path = format!("{}.tmp", file_path.display());
//     let mut temp_file = File::create(&temp_file_path)?;

//     for line in reader.lines() {
//         let line = line?;
//         if re.is_match(&line) {
//             //   ---replace the line password by entered string---
//             // 1. get the new password
//             // 1.bis encrypt the new password
//             // 2. get the line key (by split("$")
//             // 3. create line key ++ "$" ++ newpassword_encrypted
//             // 4. writeln!(temp_file, "{}", line)?

//             let new_password = input_output::new_passwd();
//             let new_encrypted_password = utils::encrypt(&new_password, crate::GLOBAL_SECRET_KEY);
//             let login_that_match = &line.split("$").next().unwrap();
//             println!("login that match : {}", login_that_match);

//             let new_line = login_that_match.to_string() + "$" + &new_encrypted_password;

//             writeln!(temp_file, "{}", new_line)?;
//             continue;
//         }
//         writeln!(temp_file, "{}", line)?;
//     }
//     drop(temp_file);

//     std::fs::remove_file(file_path.clone())?;
//     std::fs::rename(&temp_file_path, file_path)?;

//     Ok(())
// }

// fn delete_line_of_record_with_key(key: &String, file_path: &PathBuf) -> Result<(), io::Error> {
//     let file = File::open(file_path)?;
//     let reader = BufReader::new(&file);
//     let re = Regex::new(key).expect("Invalid regex pattern");

//     let temp_file_path = format!("{}.tmp", file_path.display());
//     let mut temp_file = File::create(&temp_file_path)?;

//     for line in reader.lines() {
//         let line = line?;
//         if !re.is_match(&line) {
//             writeln!(temp_file, "{}", line)?;
//         }
//     }

//     drop(temp_file);
//     std::fs::remove_file(file_path)?;
//     std::fs::rename(&temp_file_path, file_path)?;

//     Ok(())
// }

// fn get_choice() -> String {
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("failed to get a choice");
//     input.pop();
//     let choice = input.trim().to_string();
//     choice
// }

fn get_keys_matching_pattern(pattern: &str, file_path: &PathBuf) -> Vec<(String, String)> {
    let mut key = String::new();
    let mut _password = String::new();
    let mut output: Vec<(String, String)> = Vec::new();
    let f = File::open(file_path).expect("file not exists");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut split_line = line.split('$');
                let key_part = split_line.next().expect("split line with $ failed !");
                if key_part.to_lowercase().contains(&pattern.to_lowercase()) {
                    let mut iter = line.split("$");
                    if let Some(result) = iter.next() {
                        key = result.to_string();
                    }
                    if let Some(pwd) = iter.next() {
                        _password = decrypt(pwd, crate::GLOBAL_SECRET_KEY);
                        output.push((key.clone(), _password));
                    }
                }
            }
            Err(e) => println!("erreur {}", e),
        }
    }
    output
}
