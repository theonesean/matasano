use std::fs;

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
    // set_one_challenge_three();

    // CHALLENGE FOUR
    set_one_challenge_four();
}

// input has been XOR'd with one character (assumption: ASCII character)
// figure out which one it is, and crack the plaintext
fn set_one_challenge_three() {
    const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let output = matasano::set_one::xor_all_ascii(INPUT);

    // print best solution
    println!(
        "Your best solution is: '{}', with key '{}' and Englishness score of {}.",
        output[0].plaintext, output[0].key, output[0].score
    );
}

// find which string in the file has been encrypted with single-character XOR
fn set_one_challenge_four() {
    let contents = fs::read_to_string("files/4.txt").expect("Something went wrong.");
    let lines = contents.lines(); // line-by-line iterator

    // try XOR decryption for each line
    let mut solutions = Vec::new();
    struct Solution {
        candidates: Vec<matasano::set_one::Candidate>,
        index: usize,
    }

    for (i, line) in lines.enumerate() {
        let solution = Solution {
            candidates: matasano::set_one::xor_all_ascii(line),
            index: i,
        };

        solutions.push(solution);
    }

    solutions.sort_by(|a, b| {
        b.candidates[0]
            .score
            .partial_cmp(&a.candidates[0].score)
            .unwrap()
    });
    println!(
        "The highest-scoring string was line {}.\nDecrypted, the string was \n{}\nwith key '{}' and Englishness score of {}.",
        solutions[0].index,
        solutions[0].candidates[0].plaintext,
        solutions[0].candidates[0].key,
        solutions[0].candidates[0].score,
    );
}
