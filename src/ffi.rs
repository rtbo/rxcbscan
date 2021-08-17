use crate::output::Output;
use crate::xcbgen::{XcbEmit};
use crate::naming::Naming;
use std::io::{self, Write};

#[derive(Debug)]
pub struct FfiXcbEmit {
    naming: Naming,
    out: Output,
}

impl FfiXcbEmit {
    pub fn new(naming: Naming, out: Output) -> FfiXcbEmit {
        FfiXcbEmit { naming, out }
    }
}

impl XcbEmit for FfiXcbEmit {
    fn emit_proloque(&mut self) -> io::Result<()> {
        writeln!(&mut self.out, "use ffi::base::*;")?;
        writeln!(&mut self.out, "use libc::{{c_char, c_int, c_uint, c_void}};")?;
        writeln!(&mut self.out, "use std;")
    }

    fn emit_xidtype(&mut self, name: &str) -> io::Result<()> {
        writeln!(&mut self.out, "pub type {} = u32;", self.naming.ffi_type(name))
    }
}
