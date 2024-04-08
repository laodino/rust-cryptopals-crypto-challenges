use std::collections::HashMap;
use std::ops::Add;
use crate::utils::decode::convert_string_to_hex_vec;

pub fn single_byte_xor(s:String, byte:u8)->Vec<u8>{
  let vec = convert_string_to_hex_vec(s);
  let res_xor = vec.iter().map(|x|x^byte).collect();
  // let hex_res: Vec<String> = res_xor.iter().map(|&d| format!("{:02x}", d)).collect();
  // let res: String = hex_res.iter().map(ToString::to_string).collect();
  // print!("{:?}", res);
  res_xor
}

pub fn single_byte_xor_to_string(s:String, byte:u8)->String{
  let vec = convert_string_to_hex_vec(s);
  let res_xor:Vec<char> = vec.iter().map(|x|char::from(x^byte)).collect();
  let res: String = res_xor.iter().collect();
  // print!("{:?}", res);
  res
}

pub fn find_key(crypt_string:String)->(char,f32,String){
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
    let raw_vec = single_byte_xor(crypt_string.clone(),i as u8);

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
let crypt_string = single_byte_xor_to_string(crypt_string.clone(), u8::try_from(vec_score[0].0).unwrap());
 (vec_score[0].0,vec_score[0].1,crypt_string)

}