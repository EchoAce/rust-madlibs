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


pub fn fill_blanks(mut template: String) -> String {
    // this section can probably be improved...too many variables
    let mut l_iter = template.match_indices("[");
    let mut r_iter = template.match_indices("]");
    let mut l_duple = l_iter.next();
    let mut r_duple = r_iter.next();
    while l_duple != None {
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
        /*
        let mut result = String::new();
        result.push_str(unsafe{raw::slice_bytes(template, 0, l_index)});
        result.push_str(input.as_slice());
        result.push_str(unsafe{raw::slice_bytes(template, r_index, template.char_len())});
        template = result.as_slice();
        */
        template = replace(template.as_slice(), unsafe{raw::slice_bytes(template.as_slice(), l_index, r_index)}, input.as_slice());
        l_duple = l_iter.next();
        r_duple = r_iter.next();
    }
    template.to_string()
}
