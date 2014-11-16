use std::io;
use std::str::raw;
use std::str::replace;

// read_input keeps taking in lines from stdin until a single newline
// is entered. It returns a String.
pub fn read_input() -> String {
    let mut story = "".to_string();
    println!("Please enter a line of the story. Enter a blank line to stop");
    loop {
        let input = io::stdin().read_line().ok().expect("Failed to read line.");
        let line = input.as_slice();
        if line == "\n" {
            break;
        }
        else {
            story.push_str(line);
        }
    }
    story.to_string()
}


pub fn fill_blanks(s: &str) -> String {
    // this section can probably be improved...too many variables
    let mut l_iter = s.match_indices("[");
    let mut r_iter = s.match_indices("]");
    let l_duple = l_iter.next();
    let r_duple = r_iter.next();
    let l_index = match l_duple {
        Some((x, y)) => x,
        None => -1,
    };
    let r_index = match r_duple {
        Some((x, y)) => y,
        None => -1,
    };
    if l_index == -1 {
        s.to_string()
    }
    else {
        println!("Give me a/an {}", unsafe{raw::slice_bytes(s.as_slice(), l_index + 1, r_index - 1)});
        let input = std::io::stdin().read_line().ok().expect("Failed to fill blank.");
        replace(s.as_slice(), unsafe{raw::slice_bytes(s.as_slice(), l_index, r_index)}, input.as_slice())
    }
}
