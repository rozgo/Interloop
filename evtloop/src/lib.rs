
#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello world from rust - #2")
}

#[no_mangle]
pub extern "C" fn get_num() -> i32 {
    52
}

#[no_mangle]
pub extern "C" fn get_array() -> [u8; 5] {
    let xs: [u8; 5] = [1, 2, 3, 4, 5];
    xs
}

// fn mult(a:i32, b:i32) -> i32 {
//     a * b
// }

#[no_mangle]
pub extern "C" fn combine(func : extern "C" fn(i32,i32) -> i32) -> i32 {
    func(2,3)
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
