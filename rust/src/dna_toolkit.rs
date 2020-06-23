use rand::Rng;
use std::collections::HashMap;

fn _gen_random_seq(length: i32) -> String {
    let nucleuotides = vec!['A', 'C', 'G', 'T'];
    let mut rnd_str = String::new();

    for _ in 0..length {
        rnd_str.push(nucleuotides[rand::thread_rng().gen_range(0, nucleuotides.len())]);
    }
    return rnd_str;
}

fn _transcription(dna: &String) -> String {
    return dna.replace("T", "U");
}

fn _reverse_complement(dna: &String) -> String {
    // Generating a complement string and returning
    // reveresed version.
    // let mut trans_dict = HashMap::new();
    // trans_dict.insert('A', 'T');
    // trans_dict.insert('T', 'A');
    // trans_dict.insert('C', 'G');
    // trans_dict.insert('G', 'C');

    let trans_dict: HashMap<char, char> = [('A', 'T'), ('T', 'A'), ('C', 'G'), ('G', 'C')]
        .iter()
        .copied()
        .collect();

    let mut complement_dna = String::new();

    for nuc in dna.chars().rev() {
        complement_dna.push(trans_dict[&nuc]);
    }

    return complement_dna;
}
