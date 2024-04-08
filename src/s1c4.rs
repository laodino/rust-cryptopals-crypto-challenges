use std::fs::{ read_to_string};
use std::io::Read;
use crate::single_byte_xor_cipher::{find_key};

pub fn detect_single_character_xor(){
    let  content_vec =read_lines("src/cryptfile.txt");
    // println!("{:?}",content_vec);
    let mut score_vec:Vec<(String,char, f32, String)> = Vec::new();
    for crypt in content_vec{
        let res =   find_key(crypt.clone());
        // println!("crypt string {},key:{},score:{},decrypt string:{}",crypt.clone(),res.0,res.1,res.2);
        score_vec.push((crypt,res.0,res.1,res.2));
    }
    score_vec.sort_by(|a,b|b.2.total_cmp(&a.2));
    println!("crypt string {},key:{},score:{},decrypt string:{}",score_vec[0].0,score_vec[0].1,score_vec[0].2,score_vec[0].3);
}


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}