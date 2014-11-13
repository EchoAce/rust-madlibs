// Example story: He was so [adjective] that he [verb]ed John in the face,
// sending him into a/an [noun].

extern crate "rust-madlibs" as madlibs;
use madlibs::read_input;
use std::str::raw;

fn main() {
    let mut template = read_input();
    // println!("{}", template);

    // this section can probably be improved...too many variables
    let mut l_iter = template.match_indices("[");
    let mut r_iter = template.match_indices("]");
    let mut l_duple = l_iter.next();
    let mut r_duple = r_iter.next();
    while l_duple != None {
        let mut result = String::new();
        let l_index = match l_duple {
            Some((x, y)) => x,
            None => -1
        };
        let r_index = match r_duple {
            Some((x, y)) => y,
            None => -1
        };
        println!("Give me a/an {}", unsafe{raw::slice_bytes(template.as_slice(), l_index, r_index)});
        let input = std::io::stdin().read_line().ok().expect("Failed to fill blank.");
        result.push_str(unsafe{raw::slice_bytes(template.as_slice(), 0, l_index)});
        result.push_str(input.as_slice());
        result.push_str(unsafe{raw::slice_bytes(template.as_slice(), r_index, template.char_len())});
        template = result;
        l_duple = l_iter.next();
        r_duple = r_iter.next();
    }
    println!("{}", template);
}
