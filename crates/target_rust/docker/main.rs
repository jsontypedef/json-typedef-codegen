mod jtd_codegen_e2e;

use std::io::BufRead;

fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let input: jtd_codegen_e2e::MAIN = serde_json::from_str(&line).unwrap();

        println!("{}", serde_json::to_string(&input).unwrap());
    }
}
