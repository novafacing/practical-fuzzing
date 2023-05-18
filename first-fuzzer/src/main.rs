extern "Rust" {
    fn decode(encoded_input: &[u8]) -> Vec<u8>;
}

#[no_mangle]
pub extern "C" fn __sanitizer_cov_8bit_counters_init(start: *mut u8, stop: *mut u8) {
    println!(
        "initializing counters {:#x}:{:#x}",
        start as usize, stop as usize
    );
}

fn main() {
    let expected = String::from("👾 Exterminate!").as_bytes().to_vec();
    let encoded = "%F0%9F%91%BE%20Exterminate%21".as_bytes();
    let decoded = unsafe { decode(encoded) };
    println!("Got decoded: {:?}", decoded);
}
