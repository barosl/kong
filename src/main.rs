extern crate kong;
extern crate serialize;

use std::iter::count;
use std::os;
use std::io::fs;
use std::io;
use serialize::json;

const TRIES: uint = 6;

#[allow(unused_must_use)]
fn main() {
    if os::args()[1].as_slice() == "gen" {
        let sols = kong::get_seq(TRIES);

        {
            let mut fp = fs::File::create(&Path::new("sols.txt")).unwrap();
            for (sol, num) in sols.iter().zip(count(0i, 1)) {
                fp.write(format!("{} {}\n", num, sol).as_bytes());
            }
        }

        {
            let mut fp = fs::File::create(&Path::new("sols.json")).unwrap();
            fp.write(json::encode(&sols).as_bytes());
        }

        fs::chmod(&Path::new("sols.json"), io::USER_READ | io::USER_WRITE | io::OTHER_READ);
    }
}
