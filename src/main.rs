mod syn;
mod prelude;

use crate::prelude::*;
pub use syn::*;

fn main() {

    let a = 0;
    a;
    println!("Hello, world!");
}


#[test]
fn syn_tests() {
   let files = std::fs::read_dir("programs").unwrap()
        .map(|f| f.map(|e|e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

    for f in files {
        let data = std::fs::read_to_string(&f).unwrap();

        println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", data);
        match syn::parser::file(&data) {
            Ok(_) => {}
            Err(peg::error::ParseError{ location : peg::str::LineCol { line, column, offset, ..}, expected }) => {
                let l = data.lines().collect::<Vec<_>>();


                panic!("\n\n{}|||{}expected: {}\n^^^^^^^^^^^^^^^^^^^^^^" ,&data[offset - 10 .. offset] ,&data[offset.. ], expected);
            }
        }
        //let file = syn::File::parse(&data).unwrap();
        //println!("{:?} is {:#?}", &f, file);

    }
}