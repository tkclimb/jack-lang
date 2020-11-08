use std::fmt;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Meta {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Str,
    Bool,
    Nil,
    Undef,
}

impl Type {
    pub fn from(id: &str) -> Option<Self> {
        match id {
            "i8" => Some(Type::I8),
            "i16" => Some(Type::I16),
            "i32" => Some(Type::I32),
            "i64" => Some(Type::I64),
            "u8" => Some(Type::U8),
            "u16" => Some(Type::U16),
            "u32" => Some(Type::U32),
            "u64" => Some(Type::U64),
            "f32" => Some(Type::F32),
            "f64" => Some(Type::F64),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub body: StmtBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        meta: Meta,
        name: String,
        rhs: Expr,
    },
    Assign {
        meta: Meta,
        lhs: Expr,
        rhs: Expr,
    },
    FnDecl {
        meta: Meta,
        name: String,
        args: Vec<VarDecl>,
        block: StmtBlock,
    },
    // While {
    //     meta: Meta,
    //     block: StmtBlock,
    // },
}
pub type StmtBlock = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub meta: Meta,
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg<T> {
    pub meta: Meta,
    pub label: Option<String>,
    pub value: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOpKind {
    Or,
    And,
    Eq,
    Ne,
    Ge,
    Le,
    Gt,
    Lt,
    Not,
    Plus,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num {
        meta: Meta,
        value: Value,
    },
    Str {
        meta: Meta,
        value: String,
    },
    Var {
        meta: Meta,
        // ty: Type,
        name: String,
    },
    Call {
        meta: Meta,
        func: Box<Self>,
        param: Vec<CallArg<Self>>,
    },
    Nil {
        meta: Meta,
    },
    BinOp {
        meta: Meta,
        kind: BinOpKind,
        // ty: Type,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
}

// #[derive(Debug, Clone)]
// pub enum AstNode {
//     Int(i32),
//     Float(f32),
//     Str(String),
//     Nil,

//     Id(String, AstType),
// Fn: Identifer, param: Vec<Identifer>, rtype: Id, block<Statement>
//     FnDecl(Box<AstNode>, Param, StmtBlock),
// Fn: Identifer, param: Vec<Identifer>
//     Call(Box<AstNode>, Param),
//     // Struct: Identifer, Vec<member>
//     StructDecl(Box<AstNode>, StmtBlock),
//     BinaryOp(Box<AstNode>, Op, Box<AstNode>, AstType),
//     UnaryOp(Op, Box<AstNode>),
//     VarDecl(Box<AstNode>, Box<AstNode>, AstType),
//     Assignment(Box<AstNode>, Box<AstNode>),
//     // conditional, block
//     WhileStmt(Box<AstNode>, StmtBlock),
//     // conditional, T-block, F-block
//     IfStmt(Box<AstNode>, StmtBlock, StmtBlock),
//     ReturnStmt(Box<AstNode>, AstType),
// }

// pub fn typeof_ident(v: &String) -> AstType {
//     let v2 = v.to_lowercase();
//     match &v2[..] {
//         "int" => AstType::Int,
//         "float" => AstType::Float,
//         "str" => AstType::Str,
//         "bool" => AstType::Bool,
//         _ => AstType::Ext(v2),
//     }
// }

// pub fn ident_name(ident: &AstNode) -> String {
//     match ident {
//         AstNode::Id(var, _) => var.clone(),
//         _ => unreachable!("UnKnown ident: {:?}", ident),
//     }
// }

// pub fn ident_type(ident: &AstNode) -> AstType {
//     match ident {
//         AstNode::Id(_, typ) => typ.clone(),
//         _ => unreachable!(),
//     }
// }

// pub fn update_ident_type(ident: &mut AstNode, typ: AstType) {
//     if let AstNode::Id(_, ref mut _typ) = ident {
//         *_typ = typ;
//     }
// }

// pub fn nil_node(n: &AstNode) -> bool {
//     match n {
//         AstNode::Nil => true,
//         _ => false,
//     }
// }

// impl fmt::Display for AstType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl fmt::Display for AstNode {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// pub fn is_logic_op(op: Op) -> bool {
//     match op {
//         Op::EQ
//         | Op::NE
//         | Op::LE
//         | Op::GE
//         | Op::LT
//         | Op::GT
//         | Op::OR
//         | Op::AND => true,
//         _ => false,
//     }
// }

// impl fmt::Display for Op {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let s = match *self {
//             Op::OR => "||",
//             Op::ASSIGN => "=",
//             Op::AND => "&&",
//             Op::EQ => "==",
//             Op::NE => "!=",
//             Op::GE => ">=",
//             Op::LE => "<=",
//             Op::GT => ">",
//             Op::LT => "<",
//             Op::NOT => "!",
//             Op::PLUS => "+",
//             Op::SUB => "-",
//             Op::MUL => "*",
//             Op::DIV => "/",
//             _ => "UnKnown",
//         };
//         s.fmt(f)
//     }
// }
