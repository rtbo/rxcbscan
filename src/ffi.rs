use crate::output::Output;
use crate::xcbgen::{XcbEmit};
use std::io::{self, Write};

pub struct FfiXcbEmit {
    out: Output,
}

impl FfiXcbEmit {
    pub fn new(out: Output) -> FfiXcbEmit {
        FfiXcbEmit { out: out }
    }
}

impl XcbEmit for FfiXcbEmit {
    fn emit_xidtype(&mut self, name: &str) -> io::Result<()> {
        writeln!(&mut self.out, " pub type {}   = u32;", type_name(name))
    }
}

fn type_name(typ: &str) -> String {
    return format!("xcb_{}_t", typ);
}
