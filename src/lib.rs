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

    // const array of letter frequency in English text
    // source: https://crypto.stackexchange.com/a/40930
    // similar table also available on https://enwp.org/Letter_frequency
    pub const LETTER_FREQUENCIES: [f32; 27] = [
        0.0651738, 0.0124248, 0.0217339, 0.0349835, 0.1041442, 0.0197881, 0.0158610, 0.0492888,
        0.0558094, 0.0009033, 0.0050529, 0.0331490, 0.0202124, 0.0564513, 0.0596302, 0.0137645,
        0.0008606, 0.0497563, 0.0515760, 0.0729357, 0.0225134, 0.0082903, 0.0171272, 0.0013692,
        0.0145984, 0.0007836, 0.1918182, // Y, Z, space
    ];

    // determines the likelihood of a string to be English or not
    // implements the Bhattacharyya coefficient using letters and space
    pub fn englishness(sample: &str) -> f32 {
        let arg = sample.to_ascii_uppercase();
        // short-circuit if string is empty
        if arg.len() == 0 {
            return 0.0;
        };
        // get frequency table of sample string
        let mut count: [i32; 27] = [0; 27];
        // first, count characters
        for c in arg.chars() {
            // TODO: refactor to use .fold and a hashmap
            // see https://stackoverflow.com/q/60660277
            match c {
                'A'..='Z' => count[c as usize - 65] += 1,
                ' ' => count[26] += 1,
                _ => (),
            }
        }
        // then, convert from letter counts to frequencies
        let freq: [f32; 27] = count.map(|x| x as f32 / arg.len() as f32);

        // now we calculate the sum of squares
        let coefficient = freq
            .iter()
            .enumerate()
            .fold(0.0, |acc, (i, x)| acc + (x * LETTER_FREQUENCIES[i]).sqrt());
        return coefficient;
    }

    #[derive(Debug)]
    pub struct Candidate {
        pub plaintext: String,
        pub key: char,
        pub score: f32,
    }

    // For a given hex-encoded string, XORs it with all ASCII characters,
    // calculates the 'englishness' score, and returns a sorted vec.
    pub fn xor_all_ascii(test: &str) -> Vec<Candidate> {
        let input_len: usize = hex::decode(test).unwrap().len();
        let mut output = Vec::new();
        // step one: XOR input with every ASCII character
        // and calculate its "englishness" score
        for c in 0..=127 as u8 {
            // repeat test key character `input_len` times
            let test_key = &std::iter::repeat(c as char)
                .take(input_len)
                .collect::<String>();

            // get options
            let val = hex::decode(fixed_xor(test, &hex::encode(test_key))).unwrap();
            let decoded = String::from_utf8_lossy(&val).to_string();
            let englishness = englishness(&decoded);

            let c = Candidate {
                plaintext: decoded,
                key: c as char,
                score: englishness,
            };
            output.push(c);
        }

        // sort solutions by englishness
        output.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        return output;
    }

    pub fn repeating_key_xor(plaintext: &str, key: &str) -> String {
        return "Result".to_string();
    }
}
