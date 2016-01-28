
#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello world from rust - #2")
}

#[no_mangle]
pub extern "C" fn get_num() -> i32 {
    52
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
