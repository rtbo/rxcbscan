use crate::codegen::CodeGen;
use crate::output::Output;
use crate::xcbgen::Event;
use std::io::{self, Write};

#[derive(Debug)]
pub struct RustXcbEmit {
    cg: CodeGen,
    out: Output,
}

impl RustXcbEmit {
    pub fn new(cg: CodeGen, out: Output) -> RustXcbEmit {
        RustXcbEmit { cg, out }
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
        writeln!(&mut self.out, "use ffi::{}::*;", self.cg.xcb_mod())?;
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
                self.cg.emit_type_alias(
                    &mut self.out,
                    &self.cg.rust_type_name(name),
                    &self.cg.ffi_type_name(name),
                )?;

            }
            _ => {}
        }

        Ok(())
    }
}
