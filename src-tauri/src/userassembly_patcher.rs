use regex::Regex;
use std::{fs, fs::File, fs::OpenOptions, io::Read, io::Write, path::Path};

extern crate keystone;
use keystone::{Keystone, Mode, Arch, OptionType, OptionValue};

#[tauri::command]
pub fn patch_userassembly(userassembly_folder: &str) -> bool {
  let userassembly_file = &(userassembly_folder.to_owned() + "\\UserAssembly.dll");

  // check if userassembly_file exists
  /*if !Path::new(userassembly_file).exists() {
    println!("UserAssembly file not found");
    return false;
  }*/

  println!("Patching UserAssembly file: {}", userassembly_file);
  let original_key = assemble_key("<RSAKeyValue><Modulus>yYlF2xKeHRDZUSsXlLoSk/YAb2oIOwpbUeO4I+5GfWoybSpde4UnOlZgpKIDgltF3e9ST8bqIwuBxMoJTpOAnEKLNuBDdSeefHwhFqdczgeETxySwFKScmti1QRwgacrlgWglmaYCaeQrqbBceF9JbF4npi6S3+eFpw0j4rPjlE3vjh1AopaZQWAHGZI8Ixr7LDebe/uF8i7OCWXpkPKUTJnCEpyqM5H+pLN3MWRiL7mBR4XFqwKQr8J27Y3LN1iX9927hMsvAnh9PWoHzqpDTqIBF7w1ifYs3XQ3EMbf0zqc26UZXUaI5pD6qXNm3STz94SrfYqYY1R3Npz/Syaww==</Modulus><Exponent>AQAB</Exponent></RSAKeyValue>".to_string());
  println!("Original key: {:?}", original_key);

  return true;
}

fn assemble_key(key: String) -> Vec<u8> {
  // Build asm code

  // split key into 8 bytes
  let mut key_splits : Vec<String> = Vec::new();

  let mut key_remaining = key.clone();
  while !key_remaining.is_empty() {
    let (chunk, rest) = key_remaining.split_at(std::cmp::min(8, key_remaining.len()));
    key_splits.push(chunk.to_string());
    key_remaining = rest.to_string();
  }

  let mut code = String::new();

  //make asm code
  for (i, key_split) in key_splits.iter().enumerate() {
    let mut key_part = String::new();
    for p in key_split.chars().rev() {
      key_part += &format!("{:02X}", p as u8).to_string();
    }
    //println!("{}", key_part);

    code += &format!("mov rdx, {}h\n", key_part).to_string();
    code += &format!("mov [rax+{}], rdx\n", i*8).to_string();
  }

  //println!("code:\n{}", code);

  let engine = match Keystone::new(Arch::X86, Mode::MODE_64) {
    Ok(engine) => engine,
    Err(err) => panic!("Could not initialize Keystone engine: {}", err),
  };

  engine.option(OptionType::SYNTAX, OptionValue::SYNTAX_INTEL).unwrap();

  let result = match engine.asm(code, 0) {
    Ok(result) => result,
    Err(err) => panic!("Could not assemble: {}", err),
  };

  return result.bytes;
}