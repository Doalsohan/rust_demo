pub fn owner_ship(){
	let s = String::from("hello");
	takes_ownership(s);

	let x = 5;
	makes_copy(x);
}


fn takes_ownership(some_string: String) {
	println!("{}",some_string);
}

fn makes_copy(some_integer: i32) {
	println!("{}",some_integer);
}


pub fn first_word(s: &String) ->&str {
	let bytes = s.as_bytes();
	for(i,&item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}
	&s[..]
}