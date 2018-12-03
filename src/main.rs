use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", buffer);
    let mut result: i32 = 0;
    for line in buffer.lines() {
    	//let mut line: String = line.into();
    	let (sign, rest) = line.split_at(1);
    	let adjustment: i32 = rest.parse().unwrap();
    	match sign {
    		"-" => result -= adjustment,
    		"+" => result += adjustment,
    		_ => panic!("unexpected {:?}", sign)
    	}
    	println!("{:?}", sign);
    	//
    }
    println!("{:?}", result);
    Ok(())
}
