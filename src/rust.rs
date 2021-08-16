use crate::output::Output;
use crate::xcbgen::{Enum, XcbGen};
use std::io::{self, Write};

pub struct RustXcbGen {
    out: Output,
}

impl RustXcbGen {
    pub fn new(out: Output) -> RustXcbGen {
        RustXcbGen { out: out }
    }
}

impl XcbGen for RustXcbGen {
    fn emit_xidtype(&mut self, name: &str) -> io::Result<()> {
        Ok(())
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
