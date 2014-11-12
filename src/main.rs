// Example story: He was so [adjective] that he [verb]ed John in the face,
// sending him into a/an [noun].

extern crate "rust-madlibs" as madlibs;
use madlibs::read_input;

fn main() {
    let template = read_input();
    //println!("{}", template);

    loop {
        let l_bracket = regex!(r"[\[]");
        let l_pos = match l_bracket {
            Some(x, y) => x,
            None => -1,
        }
        let r_bracket = regex!(r"[\]]");
        let r_pos = match r_bracket {
            Some(x, y) => y,
            None => -1,
        }
        if l_pos == -1 {
            break;
        }
        
    }
    println!("{}", template);
}
