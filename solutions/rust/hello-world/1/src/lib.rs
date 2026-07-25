// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    let hello = "Hello, World!";
    return hello
}

fn main(){
    let hello_world = hello();
    println!("{hello_world}")
}