use pest::iterators::{Pair, Pairs};

use crate::prelude::*;

peg::parser! {
    pub grammar parser() for str {
        pub rule alpha() -> String = a:$(quiet!{['a'..='z'|'A'..='Z' ]}) {
            a.to_string()
        }
        pub rule line_rest()
          = (!"\n")*

        pub rule keyword()
          = (quiet!{"fn" / "of" / "to" / "at" / "is" / "impl" / "enum" / "and" / "where"
          / "return" / "continue" / "break" / "yield"
          }
        )

        pub rule __()
         = quiet!{[' ' | '\n']*}

        pub rule ident()
          = (!keyword() (__ quiet!{[ 'a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*} __ ))
          / expected!("identifier")

        pub rule patt()
          = __ ident() __

        pub rule vis()
         = __ "pub"? __

        pub rule path()
          = __ ident() __ ("." ident())* ("." "*")? __

        pub rule type_tuple()
          = __ "(" typename() ** "," ")" __

        pub rule type_path()
          = __ path() __ gen_clause()? __

        pub rule typename_simple()
          = __ (type_path() / type_tuple()) __

        pub rule typename()
          = __ x:$(modifier() typename()) / typename_simple()  __

        pub rule type_def()
         = __ ident() gen_clause()? where_clause()? __


        pub rule modifier()
          = __ ident() __

        pub rule attr()
         =  __ ("#" ident() line_rest()) __


        pub rule gen_clause()
         = __ "of" ( typename() ** ",") __

        pub rule where_clause()
         = __ ("where" (typename() "is" typename()) ** ",") __


        pub rule str_lit()
         = "\"" (!"\"" [_])* "\""

        pub rule int_lit()
          = ['0'..='9' ]+

        pub rule lit_expr()
          =  __ int_lit() / str_lit() __

        pub rule path_expr()
          = __ path() gen_clause()? __

        pub rule term()
          = __  (lit_expr() / path_expr())  __ {
          }


        pub rule expr() = precedence!{
            x:(@) "=" y:@ {}
            x:(@) quiet!{("==" / "!=" / "<" / "<=" / ">" / ">=")} y:@ { }
            x:(@) ("+" / "-") y:@ { }
                         "-"  y:@ { }
            --
            x:(@) ("*" / "/") y:@ { }
            --
            x:(@)  "(" expr() ** "," ")" {}
            x:(@)  "[" expr() ** "," "]" {}
            --
            ("(" expr() ")") {}
            term() { }
        }


        pub rule expr_stmt()
          = __ expr() __

        pub rule if_cont()
          = __ "else" block() __

        pub rule if_stmt()
          = __ "if" expr() block() if_cont()* __

        pub rule assign_stmt()
          = __ ("let" / "mut" / "const" ) path_expr() "=" expr() (";" / "\n") __

        pub rule for_stmt()
          = __ "for" patt() "in" expr() block() __

        pub rule loop_stmt()
          = __ "loop" block() __

        pub rule flow_stmt()
          = __ ("break" / "return" / "continue") __ expr()? __ ("to" __ ident())? __

        pub rule stmt()
          = __  (if_stmt() / for_stmt() / flow_stmt() / loop_stmt() / assign_stmt() / expr_stmt()) __

        pub rule label()
         = __ ident() ":" __

        pub rule block_item()
          = __ label() / (stmt() ";"?)  __

        pub rule block()
          = __ modifier()* "{" __ block_item()*  __ "}"  __


        pub rule fn_body()
         = __ (";" / block() )? __

        pub rule fn_arg_decl()
         = __ (modifier()* ident() typename()) / (ident() typename()) __

        pub rule fn_decl_args()
          = __ "(" (fn_arg_decl() ** ",") __ ")" __

        pub rule fn_decl()
          = __ modifier()* "fn" type_def() fn_decl_args() typename()? where_clause()? fn_body()?


        pub rule struct_body()
          = __ ( "{" vis()? ident() typename()) ** ("\n" / ",") "}" __

        pub rule struct_decl()
          = __ "struct" type_def() (";" / struct_body() / type_tuple()) __


        pub rule enum_decl()
          = __ "enum" type_def() ((ident() (struct_body() / type_tuple()) ) ** ("\n" / ","))

        pub rule type_decl()
          = __ "type" type_def() "=" typename() __

        pub rule var_spec()
          = __ ("let" / "mut" / "const") __

        pub rule type_clause()
         = __ ("is" typename()) __


        pub rule var_decl_def()
          = __ ";" / ("=" expr()) __

        pub rule var_decl()
          = __ var_spec() ident() type_clause()? var_decl_def() __


        pub rule trait_item()
          = __ fn_decl() / type_decl() __

        pub rule trait_decl()
          = __ "trait" type_def() "{" __ trait_item()* "}" __

        pub rule import()
          = __ "use" (path() ("as" ident())? ) ** "," __

        pub rule item_actual()
          = __ fn_decl() / struct_decl() / enum_decl() / trait_decl() / type_decl() / var_decl() / import() __

        pub rule item()
          = __ attr()* vis()? ( item_actual() ";"?) __

        pub rule file()
          =  __ item()* __
    }
}

#[test]
fn test_ident() {
    assert_eq!(parser::ident("aaa"), Ok(()));
    assert_eq!(parser::ident("aaa_"), Ok(()));
    assert_eq!(parser::ident("aaa2_"), Ok(()));
}

#[test]
fn test_path() {

    assert!(parser::path("").is_err());
    assert!(parser::path(".").is_err());
//    assert!(parser::path("a.").is_err());
    assert!(parser::path(".*").is_err());
    assert!(parser::path("*").is_err());
    assert_eq!(parser::path("aaa"), Ok(()));
    assert_eq!(parser::path("aaa_.*"), Ok(()));
    assert_eq!(parser::path("aaa2_.*"), Ok(()));
}

#[test]
fn test_typename() {
    assert_eq!(parser::modifier("ref"), Ok(()));

    assert_eq!(parser::path("void"), Ok(()));
    assert_eq!(parser::path("int"), Ok(()));
    assert_eq!(parser::path("map"), Ok(()));
    assert_eq!(parser::path("std.map"), Ok(()));
    assert_eq!(parser::typename("mut map.a"), Ok(()));
    assert_eq!(parser::typename("ref mut map.a"), Ok(()));

    assert_eq!(parser::type_path("void"), Ok(()));
    assert_eq!(parser::type_path("int"), Ok(()));
    assert_eq!(parser::type_path("map of int"), Ok(()));
}

#[test]
fn test_lits() {
    assert_eq!(parser::str_lit("\"aaa\""), Ok(()));
    assert_eq!(parser::int_lit("0"), Ok(()));
    assert_eq!(parser::int_lit("32"), Ok(()));
}

#[test]
fn test_expr() {
    assert_eq!(parser::expr("a"), Ok(()));
    assert_eq!(parser::expr("\"aaa\" + 32"), Ok(()));
    assert_eq!(parser::expr("1*a"), Ok(()));
    assert_eq!(parser::expr("a(b)"), Ok(()));
    assert_eq!(parser::expr("mem.layout of T()"), Ok(()));
    assert_eq!(parser::expr("mem.currentAllocator.alloc(mem.layout of T())"), Ok(()));
    assert_eq!(parser::expr("mem.currentAllocator.alloc(mem.layout of T())[0, 1]"), Ok(()));

}

#[test]
fn test_stmt() {
    assert_eq!(parser::label("Hello:"), Ok(()));
    assert_eq!(parser::block("{ mem.currentAllocator.alloc(mem.layout of T()) }"), Ok(()));
    assert_eq!(parser::block("{  }"), Ok(()));
    assert_eq!(parser::block("{return a}"), Ok(()));

    assert_eq!(parser::stmt("break a to b"), Ok(()));
    assert_eq!(parser::stmt("return a to b"), Ok(()));
    assert_eq!(parser::stmt("continue b"), Ok(()));
    assert_eq!(parser::stmt("break b to C"), Ok(()));

    assert_eq!(parser::block("{return a to b}"), Ok(()));
    assert_eq!(parser::block("{break a to b}"), Ok(()));
    assert_eq!(parser::block("{continue a to b}"), Ok(()));
    assert_eq!(parser::block("{b:continue a to b}"), Ok(()));
    assert_eq!(parser::block("{
    mem.currentAllocator.alloc(mem.layout of T())
}"), Ok(()));

}
#[test]
fn test_fn() {
    assert_eq!(parser::fn_decl(r#"fn a()"#), Ok(()));
    assert_eq!(parser::fn_decl(r#"fn a() T"#), Ok(()));
    // Parsed T as modifier and (a T) as tuple ?
    assert_eq!(parser::fn_decl(r#"fn a of T(a T) {}"#), Ok(()));
    assert_eq!(parser::fn_decl(r#"fn a of T() (T,T)"#), Ok(()));
    assert_eq!(parser::fn_decl(r#"fn alloc of T () ref T where T is Sized { mem.currentAllocator.alloc(mem.layout of T()) }"#), Ok(()));

}
#[test]
fn test_trait() {
    assert_eq!(parser::trait_decl(r#"trait Ord { }"#), Ok(()));
    assert_eq!(parser::trait_decl(r#"trait Ord of T where T is Ord{ }"#), Ok(()));
    assert_eq!(parser::trait_decl(r#"
trait Len of T {
    fn len(ref self) uint;
}"#), Ok(()));

}
#[test]
fn test_use() {
    assert_eq!(parser::import("use std.net"), Ok(()));
}
#[test]
fn test_gen_clause() {
    assert_eq!(parser::ident("int"), Ok(()));
    assert_eq!(parser::path("int"), Ok(()));
    assert_eq!(parser::typename("int"), Ok(()));
    assert_eq!(parser::typename("ref int"), Ok(()));
    assert_eq!(parser::gen_clause("of int, int"), Ok(()));
    assert_eq!(parser::gen_clause("of  int of T,  int"), Ok(()));
}

#[test]
fn test_where_clause() {
    assert_eq!(parser::where_clause("where a is Maasap of asdas"), Ok(()));
    assert_eq!(parser::where_clause("where b is Oasdas"), Ok(()));
}

#[derive(pest_derive::Parser)]
#[grammar = "../grammar.pest"]
pub struct AspParser;

/*
pub trait Detok : Sized {
    fn detok(pair : Pairs<Rule>) -> Result<Self> ;
}

#[derive(Debug)]
pub struct Tuple(pub Vec<Typename>);

#[derive(Debug)]
pub enum Typename {
    Path(Path),
    Tuple(Tuple),
}

impl Detok for Typename {
    fn detok(mut pair: Pairs<Rule>) -> Result<Self, Error> {

        let item = pair.next().unwrap();
        match item {
            Rule::type_path => {            }
        }
        panic!("{:?}", item)
    }
}

pub struct Modifiers(Vec<String>);

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

pub struct StrLit {}
pub struct IntLit {}

pub enum LitExpr {
    Str(StrLit),
    Int(IntLit)
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
 */
#[derive(Debug)]
pub struct File {}