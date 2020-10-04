use crate::prelude::*;
use pest::iterators::{Pairs, Pair};
use pest::RuleType;

#[derive(pest_derive::Parser)]
#[grammar = "../grammar.pest"]
pub struct AspParser;

pub trait Detok : Sized {
    fn detok(pair : Pairs<Rule>) -> Result<Self> ;
}



#[derive(Debug)]
pub enum Typename {
    Path(Path),
    Tuple(Tuple),
}

impl Detok for Typename {
    fn detok(mut pair: Pairs<Rule>) -> Result<Self, Error> {

        let item = pair.next().unwrap();
        match item {
            Rule::type_path => {

            }
        }
        panic!("{:?}", item)
    }
}

#[derive(Debug)]
pub struct Attr {
    name: String,
    expr: String
}

impl Detok for Attr {
    fn detok(pair: Pairs<Rule>) -> Result<Self, Error> {
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct GenItem (pub Typename);

#[derive(Debug)]
pub struct GenClause(Vec<GenItem>);

#[derive(Debug)]
pub struct Path {
    segments: Vec<String>,
    generics: GenClause
}


#[derive(Debug)]
struct WithVis<T>(pub Option<String>, pub T);

impl <T : Detok> Detok for WithVis<T> {
    fn detok(mut pair: Pairs<Rule>) -> Result<Self> {
        let mut vis = None;
        loop {
            let item = pair.peek().unwrap();
            match item.as_rule() {
                Rule::vis => {
                    vis = Some(item.as_str().to_string());
                    pair.next();
                },
                other => return Ok(Self(vis, T::detok(pair)?)),
            }
        }
    }
}

#[derive(Debug)]
struct WithAttr<T>(pub Vec<Attr>, pub T);

impl <T : Detok> Detok for WithAttr<T> {
    fn detok(mut pair: Pairs<Rule>) -> Result<Self> {
        let mut attrs = vec![];
        loop {
            let item = pair.peek().unwrap();
            match item.as_rule() {
                Rule::attr => {
                    attrs.push(Attr::detok(item.into_inner())?);
                    pair.next();
                }
                _ => {
                    return Ok(Self(attrs, T::detok(pair)?));
                }
            }
        }
    }
}

#[derive(Debug)]
struct VarDecl {

}
#[derive(Debug)]
struct FnDecl {

}
#[derive(Debug)]
struct TraitDecl {
    typename: Typename,
}

impl Detok for TraitDecl {
    fn detok(mut pair: Pairs<Rule>) -> Result<Self> {

        let typename = Typename::detok(pair.next().unwrap().into_inner())?;
        unimplemented!()
    }
}

#[derive(Debug)]
struct Impl {

}
#[derive(Debug)]
struct StructDecl {

}
#[derive(Debug)]
struct EnumDecl {

}
#[derive(Debug)]
struct TypeDecl {

}
#[derive(Debug)]
enum Item {
    VarDecl(VarDecl),
    FnDecl(FnDecl),
    TraitDecl(TraitDecl),
    Impl(Impl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    TypeDecl(TypeDecl),
}



impl Detok for Item {
    fn detok(mut pair: Pairs<Rule>) -> Result<Self> {
        let item= pair.peek().unwrap();
        match item.as_rule() {
            Rule::trait_decl => {
                return Ok(Item::TraitDecl(TraitDecl::detok(item.into_inner())?));
                pair.next();
            }
            o => { panic!("{:?}", o) }
        }
        unimplemented!()
    }
}
/*
impl Item {
    fn from_tokens(tokens : Pair<Rule>) -> Result<Self> {
        let mut pairs = tokens.into_inner();
        let item  = pairs.next().unwrap();
        match item.as_rule() {
            Rule::attr => {
            }
            Rule::trait_decl => {
                return Ok(Item::TraitDecl(TraitDecl::from_tokens(item)?));
            }
            o => { panic!("{:?}", o) }
        }
        unimplemented!()
    }
}

 */

#[derive(Debug)]
pub struct File {
    items: Vec<WithAttr<WithVis<Item>>>
}

impl File {
    fn from_tokens<'a>(tokens : Pair<Rule>) -> Result<Self> {

        let mut items = vec![];
        for rule in tokens.into_inner() {
            let item = WithAttr::detok(rule.into_inner())?;
            items.push(item);
        }
        return Ok(Self {items})
    }

}
impl File {
    pub fn parse(t: &str) -> Result<Self> {
        let parser = AspParser::parse(Rule::program, t).unwrap();

        for pair in parser {

            println!("Rule:    {:?}", pair.as_rule());
            println!("Span:    {:?}", pair.as_span());
            println!("Text:    {}", pair.as_str());
            File::from_tokens(pair);

        }
        Ok(Self { items: vec![]})

    }
}