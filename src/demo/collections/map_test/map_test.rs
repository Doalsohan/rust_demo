use std::collections::HashMap;

#[test]
fn map_test(){
    /// 获取对应key的值
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"),10);
    scores.insert(String::from("red"),50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The blue team score is : {score}");

    /// map遍历
    for(key,value) in &scores {
        println!("{key}:{value}");
    }

    /// 哈希map和所有权
    /// 对于像 i32 这样的实现了 Copy trait 的类型，
    /// 其值可以拷贝进哈希 map。对于像 String 这样拥有所有权的值，
    /// 其值将被移动而哈希 map 会成为这些值的所有者
    let mut map = HashMap::new();
    let field_name = String::from("Like Color");
    let field_value = String::from("Red");
    map.insert(field_name,field_value);

    /// move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
    // println!("{field_name}:{field_value}");

    /// 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。
    /// 但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    match scores.insert(String::from("Blue"), 25) {
        Some(val) => println!("The Origin Value : {val}"),
        None => (),
    };

    println!("{:?}", scores);



    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);



    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}