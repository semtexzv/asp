mod syn;

use syn::*;
use pest::Parser;


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
        let p = syn::AspParser::parse(Rule::program, &data).unwrap();
        println!("{:?} is {:#?}", &f, p);

    }
}