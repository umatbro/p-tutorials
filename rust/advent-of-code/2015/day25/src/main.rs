mod calc;
mod traverse;

use calc::calc_code;
use traverse::{InfinitePaper, PaperPos};

fn main() {
    let requested_pos = PaperPos::from(2978, 3083);
    let infinite_paper = InfinitePaper::new();

    let mut code = 20151125;
    for pos in infinite_paper {
        if pos == requested_pos {
            break;
        }
        code = calc_code(&code);
    }
    println!("The answer is: {}", code);
}
