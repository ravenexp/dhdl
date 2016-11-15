pub struct ModuleDecl {
    pub name: Ident,
    pub endname: Ident,
    pub entities: Vec<EntityDecl>
}

pub struct EntityDecl {
    pub name: Ident,
    pub endname: Ident,
    pub ports: Option<Vec<PortDecl>>
}

pub struct PortDecl {
    pub name: Ident,
    pub dir: Option<Direction>,
    pub class: Ident
}

pub struct WireDecl {
    pub name: Ident,
    pub class: Ident
}

// Terminals

pub enum Direction { In, Out }

pub struct Number(pub String);
pub struct Ident(pub String);
