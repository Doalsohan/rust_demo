use crate::Message;

#[derive(Debug)]
pub struct Asparagus {}


fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
     }
    largest
}

#[test]
fn test_fn() {
    let mut vect = vec![34,45,67,3,46,7];
    let num = largest(&vect);
    println!("The largest num is {num}");
}


struct ResultResp<T> {
    success: bool,
    message: String,
    code: String,
    data: T,
}

impl <T> ResultResp<T> {
    fn data(success:bool,message: String,code:String,data : T) -> ResultResp<T> {
        ResultResp {
            success,
            message,
            code,
            data
        }
    }
}



struct Point<T> {
    x:T,
    y:T,
}


impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


#[test]
fn test_point() {
    let p = Point {x :6,y:20};
    println!("p.x = {}",p.x())
}