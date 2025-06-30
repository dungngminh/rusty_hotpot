fn main() {
    // mutable and immutable
    println!("This is main");

    let x: i32 = 5;

    println!("x value is {x}");

    let mut mut_x: i32 = 5;

    println!("mut_x before changed is {mut_x}");

    mut_x += 1;

    println!("mut_x after changed is {mut_x}");

    // Control flow

    if x > 5 {
        println!("x is larger than 5")
    } else if x < 5 {
        println!("No, x is smaller than 5")
    } else {
        println!("X equals 5")
    }

    // Normal loop

    let mut counter: i32 = 0;

    let result_after_loop = loop {
        counter += 1;

        if counter >= 10 {
            break counter * 2;
        }
    };

    println!("result after normal loop is {}", result_after_loop);

    // For loop

    let arr = [1, 2, 3, 4, 5, 6, 7, 8];

    for a in arr {
        println!("value {a}")
    }

    let mut s = String::from("I'm");

    let s1 = s.clone();

    s.push_str(", Dzung");

    println!("{s}");

    println!("{s1}");

    test_fuction(&mut s);

    println!("{s}");

    let s2 = test_return_string(&mut s);

    println!("{s2}");

    let s3 = test_return_string(&mut s);

    println!("{s3}");
}

fn test_fuction(s: &mut String) {
    println!("s in test_function is {s}");

    s.push_str(", Dzung");

    println!("s in test_function is {s}");
}

fn test_return_string(s: &mut String) -> &String {
    s.push_str(", DzungNgMinh");

    s
}
