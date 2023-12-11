use std::cmp::Ordering;
use std::io;
use rand::Rng;
use crate::calculate_length;

fn show_arr_number() {
    let a = [1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please type a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}



fn guess_number() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed :{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}



fn loop_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


/**
 *
 * & 符号放在变量前面表示，对变量的引用
 *
 * 创建一个引用的行为称为借用（borrowing）
 *
 * 默认不能修改引用的值
 *
 * 同一个可变变量，只能创建一个可变引用。
 *
 *
 * */

fn rust_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}