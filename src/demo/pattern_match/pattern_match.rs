enum Direction {
    East,
    West,
    North,
    South
}

/// match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
/// match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
/// X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y
/// 可以类比 switch case default
#[test]
fn pattern_match_first() {
    let direct = Direction::South;
    match direct {
        Direction::East => println!("east"),
        Direction::North | Direction::South => {
            println!("east or south");
        },
        _ => println!("Other"),
    };
}