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
        let file = syn::File::parse(&data).unwrap();
        println!("{:?} is {:#?}", &f, file);

    }
}