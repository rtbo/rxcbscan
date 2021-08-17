use crate::output::Output;
use crate::xcbgen::XcbEmit;
use crate::naming::Naming;
use std::io::{self, Write};

#[derive(Debug)]
pub struct RustXcbEmit {
    naming: Naming,
    out: Output,
}

impl RustXcbEmit {
    pub fn new(naming: Naming, out: Output) -> RustXcbEmit {
        RustXcbEmit { naming, out }
    }
}

impl XcbEmit for RustXcbEmit {
    fn emit_proloque(&mut self) -> io::Result<()> {
        writeln!(&mut self.out, "use base;")?;
        writeln!(&mut self.out, "use ffi::base::*;")?;
        writeln!(&mut self.out, "use ffi::{}::*;", self.naming.xcb_mod())?;
        writeln!(&mut self.out, "use libc::{{self, c_char, c_int, c_uint, c_void}};")?;
        writeln!(&mut self.out, "use std;")?;
        writeln!(&mut self.out, "use std::iter::Iterator;")?;
        Ok(())
    }

    fn emit_xidtype(&mut self, name: &str) -> io::Result<()> {
        Ok(())
    }
}
