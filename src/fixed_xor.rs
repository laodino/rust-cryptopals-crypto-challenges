use crate::utils::decode::*;
pub fn fixed_xor(s1: String, s2: String) -> String {
    let vec1 = convert_string_to_hex_vec(s1);
    let vec2 = convert_string_to_hex_vec(s2);
    let res: Vec<u8> = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(h1, h2)| h1 ^ h2)
        .collect();
    let hex_res: Vec<String> = res.iter().map(|&d| format!("{:02x}", d)).collect();
    let res: String = hex_res.iter().map(ToString::to_string).collect();
    print!("{:?}", res);
    res
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let para1 = "1c0111001f010100061a024b53535009181c".into();
        let para2 = "686974207468652062756c6c277320657965".into();
        let res = fixed_xor(para1,para2);
        assert_eq!(res,"746865206b696420646f6e277420706c6179");
    }
}
