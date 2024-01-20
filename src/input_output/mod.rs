use crate::utils::{decrypt, encrypt};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// new_record create a new pair
/// new_id() get the login/key of site/computer $
/// then search wether this id exists -> then return with msg

///

// pub fn new_record(key: &Add, path: PathBuf) {
//     let repository: Vec<String> = read_data_file(path.clone());

//     match key.login {
//         Some(ref pattern) => {
//             //  test wether key exists in repository
//             for line in repository {
//                 let stored_key = line.split("$").next().unwrap();
//                 if pattern == stored_key {
//                     println!("This identifier has a stored password");
//                     return;
//                 }
//             }
//             let password = new_passwd();
//             let encrypted_password = encrypt(&password, crate::GLOBAL_SECRET_KEY);
//             let new_key_passwd = pattern.to_string()
//                 + &"$".to_string()
//                 + &encrypted_password.to_string()
//                 + &"\n".to_owned();

//             save_to_data_file(&path, new_key_passwd);
//         }

//         None => {
//             println!("I need a login")
//         }
//     }
// }

pub fn new_passwd() -> String {
    let mut passwd = String::new();
    println!("enter password ");
    io::stdin()
        .read_line(&mut passwd)
        .expect("unable to read user input ");
    passwd.pop();
    passwd
}

// pub fn new_id() -> String {
//     let mut id = String::new();
//     println!(" enter site/computer name ");
//     io::stdin()
//         .read_line(&mut id)
//         .expect("unable to read user input ");
//     id.pop();
//     let new_id = id.trim().to_string();
//     new_id
// }

pub fn save_to_data_file(path: &PathBuf, new_record: String) {
    let mut f = File::options()
        .append(true)
        .write(true)
        .read(true)
        .open(&path)
        .unwrap();
    f.write_all(new_record.as_bytes()).unwrap();
}

pub fn read_data_file(path: PathBuf) -> Vec<String> {
    let mut output_string = String::new();
    let output_vector_of_string: Vec<String>;

    if path.exists() {
        let f = File::options().read(true).open(&path).unwrap();

        let mut buffer_reader = BufReader::new(f);
        buffer_reader
            .read_to_string(&mut output_string)
            .expect("unable to read file");
    } else {
        let _ = File::create(&path);
    }
    output_vector_of_string = output_string
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    output_vector_of_string
}

pub fn data_file_to_hash(path: PathBuf) -> HashMap<String, String> {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    let f = File::open(&path).expect("error");
    let lines = BufReader::new(f).lines();
    //let password = String::new();
    let password = "29/Renenal".to_owned();
    let mut decoded = String::new();
    let mut key = String::new();

    for line in lines {
        if let Ok(current_line) = line {
            let mut splitted_line = current_line.split('$');
            if let Some(key_value) = splitted_line.next() {
                key = key_value.to_string();
            };
            if let Some(value) = splitted_line.next() {
                decoded = decrypt(value, &password);
            }
            hash_map.insert(key.to_owned(), decoded.to_owned());
        }
    }

    hash_map
}
