
#[grammar = "../grammar.pest"]
#[derive(pest_derive::Parser)]
pub struct AspParser;

pub enum ModItem {

}

pub struct Mod {
    items: Vec<ModItem>
}