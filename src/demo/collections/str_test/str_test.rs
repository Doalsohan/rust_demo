
#[test]
fn test_string() {
    let mut s1 = String::new();
    s1 = "sdfdsfsfdsf".to_string();
    println!("s1 is {s1}");

    let mut s2 = String::from("abc");
    s2.push('d');
    println!("s2 is {s2}");

    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);
    println!("s4 is {s4}");
    println!("s3 is {s3}");

    /// 定义的常量字符串是 &str类型 是不是slice(切片)
    let s1 = String::from("Hello,"); // 字符串slice对象
    let s2 = String::from("world!"); // 字符串slice对象
    let s3 = "Rust";
    let s4 = s1 + &s2 + s3;
    // println!("The s1 : {s1}");
    println!("The s2 : {s2}");
    println!("The s3 : {s3}");
    println!("The s4 : {s4}");

    /// String类型的字符串是分配在堆上的字节缓存区结构体有如下类比
    ///     struct String {
    ///        pointer: i32,   对应堆中的  char[]字节数组首地址
    ///        length: i32,
    ///        capacity: i32,
    ///    }
    ///
    ///
    ///
    let str = String::from("abcdefg");
    println!("Str 的长度是 :{}",str.len());
    println!("Str 的容量是: {}",str.capacity());
    let pointer_str = &str;
    println!("Str_Pointer 的容量是: {}",pointer_str.capacity());
    // 此语句编译无法通过，因为上述将String的指向保存在堆中的字节缓存区的地址
    // 赋值给了pointer_str，所以其值只是一个地址，不包含String的长度信息，因此无法切片
    // println!("A {}",pointer_str[0..1]);


    println!("A {}",&str[0..1]);

    ///在Rust中字符串字面量并不是先在内存中以String类型的方式存储，
    /// 然后再创建String的引用来得到一个&str。
    /// 而是以编译器在编译的时候直接将字符串字面量以硬编码的方式写入程序的二进制文件中，
    /// 当程序被加载时，字符串字面量保存中Read Only Memory 字段中。
    /// 当程序执行到let s = "hello"; ，将"hello" 赋值给变量s，s在栈上，
    /// 是直接将"hello" 的地址返回给变量s。如果有两个相同的字符串字面量，其地址是相同的。
    let s1 = "hello";
    let s2 = "hello";
    assert_eq!(*s1, *s2);
}