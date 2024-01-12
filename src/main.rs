
// rust管理内存的方式: rust 通过所有权的方式管理内存

// 栈的内存在运行时针对基础数据类型进行内存分配

// 栈中的所有数据都必须占用已知且固定的大小。

// 在编译时大小未知或大小可能变化的数据，要改为存储在堆上

// Rust 中的每一个值都有一个所有者(Owner)

// 值在任一时刻有且只有一个所有者

// 当所有者（变量）离开作用域，这个值将被丢弃

// 在 Rust 中，默认所有项（函数、方法、结构体、枚举、模块和常量）对父模块都是私有的

// 父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用它们父模块中的项。
// 这是因为子模块封装并隐藏了它们的实现详情，但是子模块可以看到它们定义的上下文


/****
 * 
 * 
 * 引用的规则
    让我们概括一下之前对引用的讨论：
        在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
        引用必须总是有效的。
 * 
 * 
 * 
 * 结构体：
 * 
 * 1. 普通结构体
 * 2. 元祖结构体
 * 3. 类单元结构体
 * 
 * */

use rand::Rng;
use rust_demo::eat_at_restaurant;

use demo::demo_test::demo_01::owner_ship;
use demo::demo_test::demo_01::first_word;

pub mod garden;
pub mod demo;

fn main() {

    owner_ship();

    let mut str = "Hello Word";

    str = "skjdhsfassafsaddjjkdsfhd";

    println!("变量作用域:{}",str);

    panic!("crash and burn");

    let str1 = str;

    println!("变量作用域:{}",str);

    println!("变量作用域:{}",str1);

    let df = str == str1;

    println!("{}",df);

    println!("{}=={}",str,str1);

    bian_and_no();

    let mut zs = User {
        active: true,
        username: String::from("zhangsan"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("hynnh@456.com"),
        ..zs
    };

    println!("用户名:{}",user2.username);

    zs.username = String::from("kjdsadjksafsa");

    println!("用户名:{}",user2.username);

    println!("用户的名字：{}",zs.username);

    let rect1 = (30,40);
    println!("This area of the rectangle is {} square pixels",area(rect1));

    let rect1 = Rectangle {
        width: 30,
        height: 60
    };
    println!("rect1 is {:#?}",rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );


    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    // let reference_to_nothing = dangle();


    // owner_ship();
    // let str_test = "Hello,jjjjjjjjjb";

    // println!("{str_test}");

    // str_test.push_str(", world");


    // println!("{}",str_test);


    let mut sbc = String::from("hello, word");

    let word = first_word(&sbc);

    println!("the first word is : {}",word);



    // let mmd = String::from("hhhhhhhe");

    // let len = calculate_length(&mmd);

    // println!("The length of '{}' is {}.", mmd, len);

    // // 变量转移

    // let mmds = mmd.clone();


    // println!("mmd:{}",mmd);


    // let mut fstr = String::from("hello");

    // fstr.push_str("world");

    // println!("{}",fstr);


    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is :{result}");

    // let haha = 3;

    // if haha < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // // 算术运算
    // let sum = 5 + 10;
    // println!("{sum}");

    // // subtraction
    // let difference = 95.5 - 4.3;
    // println!("{difference}");

    // // multiplication
    // let product = 4 * 30;
    // println!("{product}");

    // // division
    // let quotient = 56.7 / 32.2;
    // println!("{quotient}");

    // let truncated = -5 / 3; // 结果为 -1
    // println!("{truncated}");

    // // remainder
    // let remainder = 43 % 5;
    // println!("{remainder}");


    // let mut x = 5;
    // println!("The value of  x is : {x}");
    // x = 6;
    // println!("The value of  x is : {x}");

    // println!("Hello, world!");

    // let x = 6;

    // let x = x + 1;

    // {
    //     let x = x + 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    //  println!("The value of x is: {x}");


    
    //  // guess_number();

    //  show_arr_number();

    //  let c = add(5,6);
    //  println!("c is : {c}");

    let m = Message::Write(String::from("sdssffds"));
    m.call();

    let config_max = Some(3u8);
    match config_max {
        Some(max) => {
            println!("The value is : {}",max)
        },
        _=>(),
    };

    if let Some(max) = config_max {
        println!("The value is : {}",max);
    }

    eat_at_restaurant();
}



fn calculate_length(s: &String) -> usize {
    s.len()
}



fn add(a:u32,b:u32) -> u32 {
    a + b
}



fn owner_vari() {
    let x = 5;
    let y = x;

}



fn bian_and_no(){
    let mut s = String::from("mlbz");

    let s1 = &s;

    let s2 = &s;

    println!("{} and {}", s1, s2);

    let s3 = &mut s;

    println!("{}", s3);
}


fn build_user(email: String,username:String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}


#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Cmp;

struct Point(i32,i32,i32);


fn area(dimensions:(u32,u32)) -> u32 {
    dimensions.0*dimensions.1
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}


// 枚举成员合一放入任意类型的数据
enum IpAddr {
    V4(String),
    V6(String)
}


fn test_enum() {
    let ipv4 = IpAddr::V4(String::from("127.0.0.1"));
}

enum Message {
    Quit,
    Move{x: i32,y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        println!("Message Method Call");
    }
}

enum MMD<T> {
    NONE,
    Some(T),
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}


// match 类似Java语言的switch
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Lucky Quarter{:?}",state);
            100
        }
    }
}

