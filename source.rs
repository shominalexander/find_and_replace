use std::env;                      // let args: Vec<String> = env::args().collect();
use std::fs;                       // for item in fs::read_dir(folder).unwrap() {
use std::io::{BufRead, BufReader}; // let     lines = BufReader::new(file).lines();
use std::io::Write;                // file.write( text.to_string() );
use std::{ffi::OsStr, path::Path}; // if Path::new(name).extension().and_then(OsStr::to_str) == Some(extension) {

fn level(extension: &String, folder: &String, new_1: &String, new_2: &String, new_3: &String, old: &String, skip: usize, target: &String, whole: &String) {
 for item in fs::read_dir(folder).unwrap() {
  if item.as_ref().unwrap().path().is_dir() {
   level(&extension, &item.as_ref().unwrap().path().display().to_string(), &new_1, &new_2, &new_3, &old, skip, &target, &whole);

  } else { //if item.as_ref().unwrap().path().is_dir() {
   let     name = item.as_ref().unwrap().path().display().to_string();
   let     path = folder.to_owned() + "\\" + target;
   let mut same = false;

   if !target.trim().is_empty() { 
    if name == path { 
     same = true; 

    } //if name == path { 

   } else { //if !target.trim().is_empty() { 
    if Path::new(&name).extension().and_then(OsStr::to_str) == Some(extension) { 
     same = true; 

    } //if Path::new(name).extension().and_then(OsStr::to_str) == Some(extension) { 
   } //} else { //if !target.trim().is_empty() { 

   if same {
    let mut empty   = 0;
    let mut found   = 0;
    let mut index   = 0;
    let     items   = BufReader::new(fs::File::open(&name).unwrap()).lines();
    let mut skipped = 0usize;
    let mut text    = String::new();

    for item in items {
     let mut line = item.unwrap().to_string();

     if line.trim().is_empty() { empty = empty + 1; } else { empty = 0; }

     if line.find(&old.to_string()) != None {
      found = found + 1;

      if skip > 0 { if skipped < skip { found = found - 1; skipped = skipped + 1; } }

      if found > 0 && !new_1.trim().is_empty() {
       if !whole.trim().is_empty() {
        match found { 1 =>                                                                  line = new_1.to_string()
                    , 2 => if !new_2.trim().is_empty() { line = new_2.to_string(); } else { line = new_1.to_string(); }
                    , 3 => if !new_3.trim().is_empty() { line = new_3.to_string(); } else { line = new_1.to_string(); }
                    , _ => ()
                    }
       } else { //if !whole.trim().is_empty() {
        line = line.replace(&old.to_string(), &new_1.to_string());

       } //} else { //if !whole.trim().is_empty() {
      } //if found > 0 && !new_1.trim().is_empty() {
     } //if line.find(&old.to_string()) != None {

     index = index + 1;

     if empty < 2 { 
      if index > 1 { 
       text = text + &"\n".to_string() + &line; 

      } else { //if index > 1 { 
       text = line; 

      } //} else { //if index > 1 { 
     } //if empty < 2 { 
    } //for item in items {

    text = text + &"\n".to_string(); 

    if found > 0 {
     if !new_1.trim().is_empty() {
      text = text.replace(&"\n\n\n".to_string(), &"\n\n".to_string());

      println!("{:?} bytes written to {}", fs::OpenOptions::new().truncate(true).write(true).open(&name).unwrap().write(&text.as_bytes()).unwrap(), &name);

     } else { //if !new_1.trim().is_empty() {
      println!("found in {}", name);

     } //} else { //if !new_1.trim().is_empty() {
    } //if found > 0 {
   } //if same {
  } //} else { //if item.as_ref().unwrap().path().is_dir() {
 } //for item in fs::read_dir(folder).unwrap() {
} //fn level(extension: &String, folder: &String, new_1: &String, new_2: &String, new_3: &String, old: &String, skip: usize, target: &String, whole: &String) {

fn main() {
 let args: Vec<String> = env::args().collect();

 let extension = &args[1];
 let folder    = &args[2];
 let new_1     = &args[3];
 let new_2     = &args[4];
 let new_3     = &args[5];
 let old       = &args[6];
 let target    = &args[8];
 let whole     = &args[9];

 println!("new_1: {}", &new_1);

 match &args[7].parse::<usize>() { Ok(skip) => level(&extension, &folder, &new_1, &new_2, &new_3, &old,  *skip, &target, &whole)
                                 , Err(_)   => level(&extension, &folder, &new_1, &new_2, &new_3, &old, 0usize, &target, &whole)
 } //match &args[7].parse::<usize>() {
} //fn main() {