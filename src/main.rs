
#[macro_use]
extern crate pest_derive;


use pest::Parser;

#[grammar = "../grammar.pest"]
#[derive(Parser)]
pub struct AspParser;

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
        let p = AspParser::parse(Rule::program, &data).unwrap();
        println!("{:?} is {:#?}", &f, p);

    }
}