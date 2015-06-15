extern crate nboon;
use nboon::nb_get_line;
use nboon::NbStatus;

pub fn main() {
    let prompt:&'static str = "\x01\x1b[32m\x02\
                              [ケブラー会社]\
                              \x01\x1b[33m\x02$\x01\x1b[0m\x02 ";

    while nb_get_line(prompt) != NbStatus::EXIT {

    }
}
