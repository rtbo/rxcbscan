use crate::codegen::CodeGen;
use crate::output::Output;
use crate::xcbgen::Event;
use std::io::{self, Write};

#[derive(Debug)]
pub struct RustXcbEmit {
    codegen: CodeGen,
    out: Output,
}

impl RustXcbEmit {
    pub fn new(codegen: CodeGen, out: Output) -> RustXcbEmit {
        RustXcbEmit { codegen, out }
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
        writeln!(&mut self.out, "use ffi::{}::*;", self.codegen.xcb_mod())?;
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
                writeln!(
                    &mut self.out,
                    "pub type {} = {};",
                    self.codegen.rust_type(&name),
                    self.codegen.ffi_type(&name)
                )?;
            }
            _ => {}
        }

        Ok(())
    }
}
