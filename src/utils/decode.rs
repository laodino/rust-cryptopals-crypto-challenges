pub fn convert_string_to_hex_vec(raw: String) -> Vec<u8>{
    let mut bytes_vec:Vec<u8> = Vec::new();
    for i in 0..(raw.len()/2){
        let res = u8::from_str_radix(&raw[i*2..i*2+2],16);
        match res {
            Ok(v)=>{
                bytes_vec.push(v);
            },
            Err(e)=>{
                println!("Problem with hex: {}", e);
            }
        }
    }
    bytes_vec
}