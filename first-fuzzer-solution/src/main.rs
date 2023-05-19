extern "Rust" {
    fn decode(encoded_input: &[u8]) -> Vec<u8>;
}

fn main() {
    let expected = String::from("👾 Exterminate!").as_bytes().to_vec();
    let encoded = "%F0%9F%91%BE%20Exterminate%21".as_bytes();
    let decoded = unsafe { decode(encoded) };
    println!("Got decoded: {:?}", decoded);
}
