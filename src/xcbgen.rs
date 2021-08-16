use std::io;

pub struct TypAnnot {
    name: String,
    borrow: bool,
    mutable: bool,
}

pub struct Field {
    name: String,
    typ: TypAnnot,
}

pub struct EnumItem {
    name: String,
    value: u32,
}

pub struct Enum {
    name: String,
    items: Vec<EnumItem>,
}

pub trait XcbGen {
    fn emit_xidtype(&mut self, name: &str) -> io::Result<()>;
    fn emit_typedef(&mut self, oldname: &str, newname: &str) -> io::Result<()>;
    fn emit_enum(&mut self, enm: &Enum) -> io::Result<()>;
}
