fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        if buf.contains("quit") {
            break;
        } else {
            println!("buf: {buf}");
        }
    }
}
