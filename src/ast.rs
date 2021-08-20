use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DocField {
    pub name: String,
    pub text: String,
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
pub struct Doc {
    pub brief: String,
    pub text: String,
    pub fields: Vec<DocField>,
}

#[derive(Debug, Clone)]
pub struct EnumItem {
    pub id: String,
    pub name: String,
    pub value: u32,
}

#[derive(Debug, Clone)]
pub enum StructField {
    Field {
        id: String,
        name: String,
        typ: String,
        enu: Option<String>,
    },
    List {
        id: String,
        name: String,
        typ: String,
        len_expr: Expr<usize>,
    },
    Pad(usize),
    AlignPad(usize),
}

#[derive(Debug, Clone)]
pub enum Event {
    Import(String),
    Typedef {
        oldname: String,
        newname: String,
    },
    XidType(String),
    Enum {
        name: String,
        bitset: bool,
        items: Vec<EnumItem>,
        doc: Option<Doc>,
    },
    Struct {
        name: String,
        fields: Vec<StructField>,
        doc: Option<Doc>,
    },
    Ignore,
}
