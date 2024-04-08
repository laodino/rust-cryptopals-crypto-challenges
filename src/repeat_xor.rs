pub fn repeat_xor(s:&str,repeat_key:&str)->String{
      //构造等长key Vec
      let mut bytes = s.as_bytes().to_vec();
      let mut keys =repeat_key.as_bytes().to_vec();
      let mut new_keys= Vec::new();
      for i in 0..(bytes.len()/keys.len()){
         new_keys.append(&mut keys.clone());
      }
      for i in 0..(bytes.len()%keys.len()){
         new_keys.push(keys[i]);
      }
      println!("new keys :{:?}",new_keys);
      println!("bytes :{:?}",bytes);
      let res: Vec<u8> = bytes
          .iter()
          .zip(new_keys.iter())
          .map(|(h1, h2)| h1 ^ h2)
          .collect();
      println!("res :{:?}",res);
      let hex_res: Vec<String> = res.iter().map(|&d| format!("{:02x}", d)).collect();
      let res: String = hex_res.iter().map(ToString::to_string).collect();

   println!("{:?}",res);
   res
}

#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn test_repeat_xor(){
      let str =  "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
      let key = "ICE";
      let expect_res = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
      assert_eq!(repeat_xor(str,key),expect_res);
   }
}