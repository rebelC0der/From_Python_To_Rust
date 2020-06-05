use rand::Rng;

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
