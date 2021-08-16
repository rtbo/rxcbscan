use crate::ffi::FfiXcbEmit;
use crate::output::Output;
use crate::rust::RustXcbEmit;
use std::io;

// pub struct TypAnnot {
//     name: String,
//     borrow: bool,
//     mutable: bool,
// }

// pub struct Field {
//     name: String,
//     typ: TypAnnot,
// }

// pub struct EnumItem {
//     name: String,
//     value: u32,
// }

// pub struct Enum {
//     name: String,
//     items: Vec<EnumItem>,
// }

pub trait XcbEmit {
    fn emit_xidtype(&mut self, name: &str) -> io::Result<()>;
}

pub struct XcbGen {
    emit: (FfiXcbEmit, RustXcbEmit),
}

impl XcbGen {
    pub fn new(emit: (FfiXcbEmit, RustXcbEmit)) -> XcbGen {
        XcbGen { emit }
    }
}

impl XcbEmit for XcbGen {
    fn emit_xidtype(&mut self, name: &str) -> io::Result<()> {
        self.emit.0.emit_xidtype(name)?;
        self.emit.1.emit_xidtype(name)?;
        Ok(())
    }
}
