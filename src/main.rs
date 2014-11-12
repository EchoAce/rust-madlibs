// Example story: He was so [adjective] that he [verb]ed John in the face,
// sending him into a/an [noun].

extern crate "rust-madlibs" as madlibs;
use madlibs::read_input;
use std::str::raw;

fn main() {
    let mut template = read_input();
    //println!("{}", template);

    // adapted somewhat from replace
    for (l_start, l_end) in template.match_indices("[") {
        let mut result = String::new();
        //result.push_str(unsafe{raw::slice_bytes(s, 0, l_start)});
        println!("{}", unsafe{raw::slice_bytes(template.as_slice(), 0, l_start)});
    }
    println!("{}", template);
}
