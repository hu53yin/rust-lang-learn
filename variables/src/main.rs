fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess number: {}", guess);

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = plus_one(5);

    println!("The value of x is: {}", x);

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number is: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {}", result);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
