
use bitvec::prelude::*;
use crate::utils::decode::*;

// 将原始数据每三个字节作为一组，每个字节是8个bit，所以一共是 24 个 bit
// 将 24 个 bit 分为四组，每组 6 个 bit
// 在每组前面加补 00，将其补全成四组8个bit
// 到此步，原生数据的3个字节已经变成4个字节了，增大了将近30%
// 根据Base64码表得到扩展后每个字节的对应符号（见上图）
pub fn convert_hex_to_base64(raw: String) -> Result<String,&'static str> {
    if (raw.len()%2)!=0{
       return Err("长度不是2的倍数");
    }
    let bytes_vec = convert_string_to_hex_vec(raw);

    let mut bv = BitVec::<_, Msb0>::from_vec(bytes_vec);
    let mut res_size = (bv.len() / 6 ) as usize;
    let  equal_num = match bv.len()%6 {
        4=>{
            res_size+=1;
            bv.append(&mut bitvec![u16, Msb0; 0; 2]);
            1
        },
        2=>{
            res_size+=1;
            bv.append(&mut bitvec![u16, Msb0; 0; 4]);
            2
        },
        _ =>{
            0
        }
    };

    let mut res: Vec<char> = Vec::new();
    for i in 0..res_size {
        let start = (i * 6) as usize;
        let end = ((i + 1) * 6) as usize;
        let mut vec6 = bv[start..end].to_bitvec();
        let mut prepend_vec = bitvec![u8, Msb0; 0; 2];
        prepend_vec.append(&mut vec6);
        res.push(convert_rfc4648_to_ascii(prepend_vec.load::<u8>()));
    }
    res.append(&mut vec!['='; equal_num]);
    Ok(res.iter().collect::<String>())
}





fn convert_rfc4648_to_ascii(n: u8) -> char {
    match n {
        0..=25 => char::from(n + 65),
        26..=51 => char::from(n + 71),
        52..=61 => char::from(n - 4),
        62 => '+',
        63 => '/',
        _ => panic!("invalid rfc4648 number"),
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn  hex_to_base64(){
        let string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".into();
       let res =  convert_hex_to_base64(string);
        assert_eq!(res.unwrap(),"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

    #[test]
    fn  hex_to_base64_single(){
        let string = "41".into();
        let res =  convert_hex_to_base64(string);
        assert_eq!(res.unwrap(),"QQ==");
    }

    #[test]
    fn  hex_to_base64_double(){
        let string = "4243".into();
        let res =  convert_hex_to_base64(string);
        assert_eq!(res.unwrap(),"QkM=");
    }

}