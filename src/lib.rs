extern crate base64;
extern crate bytebuffer;
extern crate hex;

pub mod set_one {
    pub fn hex_to_base64(string: &str) -> String {
        let arg = string.to_string();
        let bytes = hex::decode(arg).unwrap();
        return base64::encode(&bytes);
    }

    // takes the XOR of two equal length buffers
    pub fn fixed_xor(lhs: &str, rhs: &str) -> String {
        // decode hex to a byte vector and test length
        // (not sure, but i think this may not be foolproof
        // in the case the last index in the vector
        // are not of equal length, but who cares)
        let buff_one = hex::decode(lhs.to_string()).unwrap();
        let buff_two = hex::decode(rhs.to_string()).unwrap();
        assert_eq!(buff_one.len(), buff_two.len());

        // xor the buffers
        let mut xor_buff = Vec::new();
        for i in 0..=buff_one.len() - 1 {
            xor_buff.push(buff_one[i] ^ buff_two[i])
        }

        return hex::encode(&xor_buff).to_string();
    }
}
