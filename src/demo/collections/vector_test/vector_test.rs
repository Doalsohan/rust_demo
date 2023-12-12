use crate::demo::collections::vector_test::vector_test::SpreadsheetCell::Int;

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
}



#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}