pub struct ModuleDecl {
    pub name: Ident,
    pub endname: Ident,
    pub entities: Vec<EntityDecl>
}

pub struct EntityDecl {
    pub name: Ident,
    pub endname: Ident,
    pub generics: Option<Vec<GenericDecl>>,
    pub ports: Option<Vec<PortDecl>>,
    pub wires: Vec<WireDecl>,
    pub insts: Vec<EntityInst>
}

pub struct GenericDecl {
    pub name: Ident,
    pub gentype: Ident
}

pub struct PortDecl {
    pub name: Ident,
    pub dir: Direction,
    pub class: Ident
}

pub struct WireDecl {
    pub name: Ident,
    pub class: Ident
}

pub struct EntityInst {
    pub name: Ident,
    pub entity: Ident,
    pub generics: Option<Vec<GenericAssign>>,
    pub ports: Option<Vec<PortAssign>>
}

pub struct GenericAssign {
    pub generic: Ident,
    pub value: GenExpr
}

pub struct PortAssign {
    pub port: Ident,
    pub dir: Direction,
    pub wire: Ident
}

pub enum GenExpr {
    Number(Number),
    String(String)
}

// Terminals

pub enum Direction { None, In, Out }

pub struct Number(pub String);
pub struct Ident(pub String);
