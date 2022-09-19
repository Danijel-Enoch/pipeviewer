fn main() {
    let hexadecimal = 0x10;
    let octal = 0o10;
    let binary = 0b10;
    let mut n = 10;
    let arr = [hexadecimal, octal, binary];

    for i in 0..arr.len() {
        n = arr[i];
        print!("{}\n", n);
    }

    //for i in 1..
}
