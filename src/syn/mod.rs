
#[grammar = "../grammar.pest"]
#[derive(pest_derive::Parser)]
pub struct AspParser;

struct VarDecl {

}

struct FnDecl {

}

struct TraitDecl {

}

struct Impl {

}

struct StructDecl {

}

struct EnumDecl {

}

struct TypeDecl {

}

pub enum ModItem {
    VarDecl(VarDecl),
    FnDecl(FnDecl),
    TraitDecl(TraitDecl),
    Impl(Impl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    TypeDecl(TypeDecl),
}

pub struct Mod {
    items: Vec<ModItem>
}