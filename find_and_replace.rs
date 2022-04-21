use std::env;                      // let args: Vec<String> = env::args().collect();
use std::fs;                       // for item in fs::read_dir(folder).unwrap() {
use std::io::Read;                 // file.unwrap().read_to_string(&mut text);
use std::io::Write;                // file.write( text.to_string() );
use std::{ffi::OsStr, path::Path}; // if Path::new(name).extension().and_then(OsStr::to_str) == Some(extension) {

fn level(extension: &String, folder: &String, new: &String, old: &String) {
 for item in fs::read_dir(folder).unwrap() {
  if item.as_ref().unwrap().path().is_dir() {
   level(&extension.to_string(), &item.as_ref().unwrap().path().display().to_string(), &new.to_string(), &old.to_string());

  } else { //if item.as_ref().unwrap().path().is_dir() {
   let     name = &item.as_ref().unwrap().path().display().to_string();
   let mut file = fs::File::open(name);
   let mut text = String::new();

   if Path::new(name).extension().and_then(OsStr::to_str) == Some(extension) {
    file.unwrap().read_to_string(&mut text);

    if text.find(&old.to_string()) != None {
     println!("{}", name);

     if !&new.to_string().trim().is_empty() {
      text = text.replace(&old.to_string(), &new.to_string());

      file = fs::OpenOptions::new().truncate(true).write(true).open(name);

      file.unwrap().write(&text.to_string().as_bytes());
     } //if !&new.to_string().trim().is_empty() {
    } //if text.find(&old.to_string()) != None {
   } //if Path::new(name).extension().and_then(OsStr::to_str) == Some(extension) {
  } //} else { //if item.as_ref().unwrap().path().is_dir() {
 } //for item in fs::read_dir(folder).unwrap() {
} //fn level(extension: &String, folder: &String, new: &String, old: &String) {

fn main() {
 let args: Vec<String> = env::args().collect();

 let extension = &args[1];
 let folder    = &args[2];
 let new       = &args[3];
 let old       = &args[4];

 level(&extension.to_string(), &folder.to_string(), &new.to_string(), &old.to_string());
} //fn main() {