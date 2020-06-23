fn _hash_maps() {
    let mut test_hm = HashMap::new();
    test_hm.insert("Key_1", vec!["Value_1"]);
    test_hm.insert("Key_2", vec!["Value_2"]);
    test_hm.insert("Key_3", vec!["Value_1", "Value_2"]);

    // println!("{}", test_hm["Key_1"]);
    println!("{:?}", test_hm["Key_1"]);

    if test_hm.contains_key("Key_3") {
        println!("{:?}", test_hm["Key_3"]);
        println!("{:?}", test_hm["Key_3"][1]);
    }

    for (key, value) in &test_hm {
        println!("{} {:?}", key, value);
    }
}
