fn main() {
    // ======SET ONE======

    // CHALLENGE ONE
    assert_eq!(
        matasano::set_one::hex_to_base64(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
        ),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );

    // CHALLENGE TWO
    assert_eq!(
        matasano::set_one::fixed_xor(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965"
        ),
        "746865206b696420646f6e277420706c6179"
    );

    // CHALLENGE THREE
    set_one_challenge_three();
}

// input has been XOR'd with one character (assumption: ASCII character)
// figure out which one it is, and crack the plaintext
fn set_one_challenge_three() {
    const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_len: usize = hex::decode(INPUT).unwrap().len();

    #[derive(Debug)]
    struct Candidate {
        plaintext: String,
        key: char,
        score: f32,
    }
    let mut output = Vec::new();

    // step one: XOR input with every ASCII character
    // and calculate its "englishness" score
    for c in 0..=127 as u8 {
        // repeat test key character `input_len` times
        let test_key = &std::iter::repeat(c as char)
            .take(input_len)
            .collect::<String>();

        // get options
        let val = hex::decode(matasano::set_one::fixed_xor(INPUT, &hex::encode(test_key))).unwrap();
        let decoded = String::from_utf8(val).unwrap();
        let englishness = matasano::set_one::englishness(&decoded);

        let c = Candidate {
            plaintext: decoded,
            key: c as char,
            score: englishness,
        };
        output.push(c);
    }

    // sort solutions by englishness
    output.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    // print best solution
    println!(
        "Your best solution is: '{}', with key '{}' and Englishness score of {}.",
        output[0].plaintext, output[0].key, output[0].score
    );
}
