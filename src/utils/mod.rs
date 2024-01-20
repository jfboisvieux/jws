use dirs::home_dir;
use magic_crypt::MagicCryptTrait;
use std::path::PathBuf;

pub fn set_path(secret_file_name: String) -> PathBuf {
    let home_path = home_dir().unwrap().join(".config");
    let relative_path = PathBuf::from(secret_file_name);
    let path = home_path.join(relative_path);

    path
}

pub fn encrypt(input: &str, secret_key: &str) -> String {
    let mc = magic_crypt::new_magic_crypt!(secret_key, 256);
    let output = mc.encrypt_str_to_base64(input);
    output
}

pub fn decrypt(input: &str, encoded: &str) -> String {
    //    println!("\n\n in decrypt {}  {}\n\n", input, encoded);
    let mc = magic_crypt::new_magic_crypt!(encoded, 256);
    let output = mc.decrypt_base64_to_string(input).unwrap();
    output
}
