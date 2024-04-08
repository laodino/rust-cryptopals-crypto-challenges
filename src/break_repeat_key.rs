use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::BufRead;
use std::ops::Add;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use bitvec::slice::Chunks;
use crate::single_byte_xor_cipher::{single_byte_xor, single_byte_xor_to_string};

pub fn hamming_distance(s1:Vec<u8>, s2:Vec<u8>) ->u32{
    let mut bv1 = BitVec::<_, Msb0>::from_vec(s1);
    let mut bv2 = BitVec::<_, Msb0>::from_vec(s2);
    let mut sum:u32 = 0;
    let mut range = bv1.len();
    if range>bv2.len(){
        range = bv2.len();
    }

    for i in 0..range{
        if bv1[i].ne(&bv2[i]){
            sum += 1;
        }
    }
    sum+=(bv1.len() as u32).abs_diff(bv2.len() as u32);
    sum
}


pub fn base64_encode(str:&str)->Vec<u8>{
    let mut raw_vec = Vec::new();
    let bytes:Vec<char> = str.chars().collect();

    for byte in bytes{
        if byte!='='{
            raw_vec.push(convert_ascii_to_rfc4648(byte));
        }
    }
    // print!("{:?}",raw_vec);
    let mut bv = BitVec::<_, Msb0>::from_vec(raw_vec);
    //println!("raw bit vec {:?}",bv);
    if (bv.len()%8) !=0{
        panic!("not 8");
    }
    let num = bv.len()/8;
    for i in 0..num{
        // println!("{:?},{:?}",i*6,i*6);
        bv.remove(i*6);
        bv.remove(i*6);
        // println!("{:?}",bv);
    }
    //println!("removed raw bit vec {:?}",bv);
    let num_u8 = bv.len()%8;
    // println!("{}",num_u8);
    if(num_u8!=0){
        for i in 0..num_u8{
            bv.remove(bv.len()-1);
        }
    }
    let res_vec  = bv.into_vec();
    res_vec
    // println!("{:?}",res_vec);
    //  let hex_res: Vec<String> = res_vec.iter().map(|&d| format!("{:02x}", d)).collect();
    //  let res: String = hex_res.join("");
    //  res
}

fn convert_ascii_to_rfc4648(c:char) -> u8 {
    match c{
        '+'=>62,
        '/'=>63,
        _=>{
            let n = u8::try_from(c).unwrap();
            match n {
                65..=90=> n-65,
                97..=132=>n-71,
                48..=57=>n+4,
                _ => {
                    println!("{}",n);
                    panic!("invalid ascii");
                }
            }
        }
    }

}

fn keysize_hamming_distance(raw:Vec<u8>,key_size:u8)->u32{
    let mut vec = Vec::new();
    let mut chunks =  raw.chunks(key_size as usize);
    loop {
        match chunks.next() {
            Some(a) => {
                vec.push(a.to_vec());
            }
            None => {
                break;
            }
        }
    }
    let mut sum:u32 = 0;
    for i in 0..vec.len()-1{
        sum+=hamming_distance(vec[i].clone(), vec[i + 1].clone());
    }
    sum
}

pub fn find_key_size(raw_content:Vec<u8>)->u32{
    let mut vec_res:Vec<(u32, u32)> = Vec::new();
    for i in 2..=40{
        let hamming_distance = keysize_hamming_distance(raw_content.clone(),i);
        // println!("key_size {},hamming distance:{:?}",i,hamming_distance);
        vec_res.push((i as u32,hamming_distance));
    }
    vec_res.sort_by(|a,b|a.1.cmp(&b.1));
    vec_res.first().unwrap().0
}

pub fn find_xor_key(v:Vec<u8>)->(char,f32){
    let mut map_char:HashMap<char,f32> = HashMap::new();
    map_char.insert(' ',18.182);
    map_char.insert('a',8.167);
    map_char.insert('b',1.492);
    map_char.insert('c',2.782);
    map_char.insert('d',4.253);
    map_char.insert('e',12.702);
    map_char.insert('f',2.228);
    map_char.insert('g',2.015);
    map_char.insert('h',6.094);
    map_char.insert('i',6.966);
    map_char.insert('j',0.153);
    map_char.insert('k',0.772);
    map_char.insert('l',4.025);
    map_char.insert('m',2.406);
    map_char.insert('n',6.749);
    map_char.insert('o',7.507);
    map_char.insert('p',1.929);
    map_char.insert('q',0.095);
    map_char.insert('r',5.987);
    map_char.insert('s',6.327);
    map_char.insert('t',9.056);
    map_char.insert('u',2.758);
    map_char.insert('v',0.978);
    map_char.insert('w',2.360);
    map_char.insert('x',0.150);
    map_char.insert('y',1.974);
    map_char.insert('z',0.074);

    let mut vec_score:Vec<(char,f32)> =Vec::new();

    let mut score:f32 = 0.;

    for i in 0..127{
        score = 0.;
        let raw_vec:Vec<u8> = v.iter().map(|x|x^i).collect();
        for c in raw_vec {
            let  s = char::from(c).to_ascii_lowercase();
            if map_char.contains_key(&s){
                let value = map_char.get(&s).unwrap();
                score=score.add(value);
            }
        }
        let char = char::from(i);
        vec_score.push((char,score));
        // println!("char {} score is {:?}",char,score);
    }
    vec_score.sort_by(|a,b| b.1.total_cmp(&a.1));
    println!("char {} score is {:?}",vec_score[0].0,vec_score[0].1);
    (vec_score[0].0,vec_score[0].1)

}

pub fn repeat_vec_xor(bytes:Vec<u8>,repeat_key:&str)->String{
    //构造等长key Vec
    let mut keys =repeat_key.as_bytes().to_vec();
    let mut new_keys= Vec::new();
    for i in 0..(bytes.len()/keys.len()){
        new_keys.append(&mut keys.clone());
    }
    for i in 0..(bytes.len()%keys.len()){
        new_keys.push(keys[i]);
    }
   // println!("new keys :{:?}",new_keys);
   // println!("bytes :{:?}",bytes);
    let res: Vec<u8> = bytes
        .iter()
        .zip(new_keys.iter())
        .map(|(h1, h2)| h1 ^ h2)
        .collect();
    //println!("res :{:?}",res);
    let hex_res: Vec<char> = res.iter().map(|&d| char::from(d)).collect();
    let res: String = hex_res.iter().collect::<String>();

   // println!("{:?}",res);
    res
}
pub fn find_repeat_key()->(String,String){
    let content:Vec<String> =read_to_string("src/repeat_key.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    //println!("{:?}",content);
    let raw_content = base64_encode(content.join("").as_str());
    let key_size = find_key_size(raw_content.clone());
    let m = raw_content.len()/key_size as usize;
    let r = raw_content.len()%key_size as usize;
    let mut res:Vec<char> = Vec::new();
    for i in 0..key_size{
        let mut vec_row:Vec<u8> = Vec::new();
        for j in 0..m{
            vec_row.push(raw_content[j*key_size as usize +i as usize])
        }
        if i<r as u32{
            vec_row.push(raw_content[(m-1)*key_size as usize+i as usize]);
        }
        //println!("{:?}",vec_row);
        let c =  find_xor_key(vec_row).0;
        res.push(c);
    }
   let key =  res.iter().collect::<String>();
    let text = repeat_vec_xor(raw_content,key.as_str());
    (key,text)
}