use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DocField {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub brief: String,
    pub text: String,
    pub fields: Vec<DocField>,
}

#[derive(Debug, Clone)]
pub enum Expr<T>
where
    T: FromStr,
    T: Clone,
{
    FieldRef(String),
    ParamRef(String),
    Value(T),
    Op(String, Box<Expr<T>>, Box<Expr<T>>),
    Unop(String, Box<Expr<T>>),
    Popcount(Box<Expr<T>>),
}

#[derive(Debug, Clone)]
pub struct XidUnion {
    pub name: String,
    pub xidtypes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EnumItem {
    pub id: String,
    pub name: String,
    pub value: u32,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub items: Vec<EnumItem>,
    pub doc: Option<Doc>,
}

#[derive(Debug, Clone)]
pub enum StructField {
    Field {
        name: String,
        typ: String,
        enu: Option<String>,
    },
    List {
        name: String,
        typ: String,
        len_expr: Expr<usize>,
    },
    ListNoLen {
        name: String,
        typ: String,
    },
    Pad(usize),
    AlignPad(usize),
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
    pub doc: Option<Doc>,
}

// a copy type for error or event
#[derive(Debug, Clone)]
pub struct OpCopy {
    pub number: i32,
    pub name: String,
}

pub type OpCopyMap = HashMap<String, Vec<OpCopy>>;

#[derive(Debug, Clone)]
pub enum Event {
    Error(i32, Struct),
    ErrorCopy {
        name: String,
        number: i32,
        ref_: String,
    },
    Import(String),
    Typedef {
        oldname: String,
        newname: String,
    },
    XidType(String),
    XidUnion(XidUnion),
    Enum(Enum),
    Struct(Struct),
    Union(Struct),
    Event(i32, Struct),
    EventCopy {
        name: String,
        number: i32,
        ref_: String,
    },
    Ignore,
}
