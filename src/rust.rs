use crate::naming::Naming;
use crate::output::Output;
use crate::xcbgen::{Event};
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

    pub fn emit_proloque(&mut self) -> io::Result<()> {
        writeln!(&mut self.out, "use base;")?;
        writeln!(&mut self.out, "use ffi::base::*;")?;
        writeln!(&mut self.out, "use ffi::{}::*;", self.naming.xcb_mod())?;
        writeln!(
            &mut self.out,
            "use libc::{{self, c_char, c_int, c_uint, c_void}};"
        )?;
        writeln!(&mut self.out, "use std;")?;
        writeln!(&mut self.out, "use std::iter::Iterator;")?;
        writeln!(&mut self.out, "")?;
        Ok(())
    }


    pub fn event(&mut self, ev: &Event) -> io::Result<()> {
        match ev {
            Event::XidType(name) => {
                writeln!(
                    &mut self.out,
                    "pub type {} = {};",
                    self.naming.rust_type(&name),
                    self.naming.ffi_type(&name)
                )?;
                writeln!(&mut self.out, "")?;
            }
            _ => {}
        }

        Ok(())
    }
}
