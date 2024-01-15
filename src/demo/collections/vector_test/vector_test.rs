use std::ops::Add;
use crate::demo::collections::vector_test::vector_test::SpreadsheetCell::Int;

// 引用确保指向某个特定类型的有效值
// 对象的引用不获取值的所有权

#[test]
fn test_vector() {
    let mut v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    v.push(10);


    //  不可变遍历
    for i in v[0..].iter() {
        println!("{}", i);
    }

    // 不可变遍历
    for i in &v {
        println!("{i}");
    }


    // 可变遍历
    for i in &mut v {
        *i *= 10;
        println!("{i}")
    }

    let one: Option<&i32> = v.get(6);
    match one {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    // let six: &i32 = &v[100];
    /// vector 删除
    // assert_eq!(v.remove(1),3);

    let row = vec![Int(3),SpreadsheetCell::Float(23.7),SpreadsheetCell::Text(String::from("dfdsfdsfds"))];

    for i in &row {
        match i {
            Int(ele) => {
                println!("The element is {}",ele);
            },
            SpreadsheetCell::Float(ele) => {
                println!("{}",ele);
            },
            SpreadsheetCell::Text(ele) => {
                println!("{}",ele);
            }
            _=>(),
        }
    }


    for i in row.iter() {
        println!("The element is {:?}",i)
    }


    let mut hqm = vec![1,2,3,4,5];
    let one = &hqm[99];
    println!("The One Is : {one}");
}



#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[test]
fn test_ref() {
    let s1 = String::from("Rust Learn");
    let len = get_str_len(&s1);
    println!("The Length of '{}' is {}.",s1,len);
    let s2 = concat_str(s1);
    println!("Result String {}",s2)
}


fn get_str_len(s: &String) -> usize {
    s.len()
}

fn concat_str(s: String) -> String{
    let t = String::from("sdsadsa");
    s.add(&t)
}
