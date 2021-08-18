use crate::codegen::{self as cg, Naming};
use crate::output::Output;
use crate::xcbgen::Event;
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

    pub fn prologue(&mut self, imports: &Vec<String>) -> io::Result<()> {
        writeln!(
            &mut self.out,
            "use libc::{{self, c_char, c_int, c_uint, c_void}};"
        )?;
        writeln!(&mut self.out, "use std;")?;
        writeln!(&mut self.out, "use std::iter::Iterator;")?;
        writeln!(&mut self.out, "")?;

        writeln!(&mut self.out, "use base;")?;
        for imp in imports.iter() {
            writeln!(&mut self.out, "use {};", imp)?;
        }
        writeln!(&mut self.out, "use ffi::base::*;")?;
        writeln!(&mut self.out, "use ffi::{}::*;", self.naming.xcb_mod())?;
        for imp in imports.iter() {
            writeln!(&mut self.out, "use ffi::{}::*;", imp)?;
        }
        writeln!(&mut self.out, "")?;

        Ok(())
    }

    pub fn epilogue(&mut self) -> io::Result<()> {
        Ok(())
    }

    pub fn event(&mut self, ev: &Event) -> io::Result<()> {
        match ev {
            Event::XidType(name) => {
                writeln!(&mut self.out, "")?;
                cg::type_alias(
                    &mut self.out,
                    &self.naming.rust_type(&name),
                    &self.naming.ffi_type(&name),
                )?;
            }
            _ => {}
        }

        Ok(())
    }
}
