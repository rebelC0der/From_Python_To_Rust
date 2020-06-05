fn _strings() {
    let mut test_str = String::from("Doom");
    println!("{}", test_str);

    test_str.push_str(" III");
    println!("{}", test_str);

    test_str.push('!');
    println!("{}", test_str);

    let p1 = String::from("Duke");
    let p2 = String::from(" Nukem");

    // let p3 = p1 + &p2; // Will generate an error
    let p3 = String::from(format!("{}{}", p1, p2));
    println!("{} {} {}", p1, p2, p3);

    // Print each character:
    for ch in p3.chars() {
        println!("{}", ch);
    }

    // Print each character and it's index:
    for (pos, ch) in p3.char_indices() {
        println!("{} - {}", pos, ch);
    }

    // Print a slice of the string
    println!("{}", &p3[0..5]);
}
