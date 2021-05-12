use std::io;

fn main() {
    println!("########## Problem. 1 ##########");

    let mut sum: i32 = 0;
    for i in 1..101 {
        sum += i;
    }
    println!("Sum from 1 to 100 is: {}", sum);

    println!("\n########## Problem. 2 ##########");

    println!("Input string: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Cannot read input!");

    
    println!("The reversed input string is: {}", input.chars().rev().collect::<String>());

    println!("\n########## Problem. 3 ##########");

    loop {
        println!("Input number: ");
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Cannot read input!");

        num = num.trim().to_string();

        // Delete leading zeors
        while num.chars().nth(0).unwrap() == '0' && num.chars().count() > 1 {
            num = (&num[1..]).to_string();
        }

        let number: u128 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[ERROR] Please type a valid number!");
                continue;
            }
        };
        if num.chars().count() > 15 {
            println!("[ERROR] The number should be no greater than 15-digit.");
            continue;
        }
        let mut cnt = 1;
        let mut formatted_str = String::new();
        for c in num.chars().rev() {
            formatted_str.insert(0, c);
            if cnt == num.chars().count() {
                break;
            }
            if cnt % 3 == 0 {
                formatted_str.insert(0, ',');
            }
            cnt += 1;
        }
        println!("{:0>20}", formatted_str);

        break;
    }
}
