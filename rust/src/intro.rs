fn _intro() {
    let x = 100.0;
    let mut y = 1.0;

    println!("x: {}, and y: {}", x, y);
    y = y * -0.3145;
    println!("x: {}, and y: {}", x, y);
    if y < x {
        println!("The difference is: {}", (x - y));
    } else if y == x {
        println!("The difference is: {}", (y - x));
    } else {
        println!("The difference is: {}", (y - x));
    }

    // let mut x = 5;
    // while x > 0 {
    //     println!("x is {}", x);
    //     x -= 1;
    // }

    let num_range = 3..7;
    for i in num_range {
        println!("i is {}", i);
    }

    let tmp_vec = vec!["DNA", "RNA", "mRNA"];
    for (pos, value) in tmp_vec.iter().enumerate() {
        println!("value at pos {} is {}", pos, value);
    }

    println!("{}", tmp_vec[0]);
}
