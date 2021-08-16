use crate::output::Output;
use crate::xcbgen::{Enum, XcbGen};
use std::io::{self, Write};

pub struct FfiXcbGen {
    out: Output,
}

impl FfiXcbGen {
    pub fn new(out: Output) -> FfiXcbGen {
        FfiXcbGen { out: out }
    }
}

impl XcbGen for FfiXcbGen {
    fn emit_xidtype(&mut self, name: &str) -> io::Result<()> {
        writeln!(&mut self.out, "pub type {} = u32;", type_name(name))
    }
    fn emit_typedef(&mut self, oldname: &str, newname: &str) -> io::Result<()> {
        Ok(())
    }
    fn emit_enum(&mut self, enm: &Enum) -> io::Result<()> {
        Ok(())
    }
}

fn type_name(typ: &str) -> String {
    return format!("xcb_{}_t", typ);
}
