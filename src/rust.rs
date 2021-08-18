use crate::codegen::CodeGen;
use crate::output::Output;
use crate::xcbgen::Event;
use std::io::{self, Write};

#[derive(Debug)]
pub struct RustXcbEmit {
    out: Output,
}

impl RustXcbEmit {
    pub fn new(out: Output) -> RustXcbEmit {
        RustXcbEmit { out }
    }

    pub fn prologue(&mut self, cg: &mut CodeGen, imports: &Vec<String>) -> io::Result<()> {
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
        writeln!(&mut self.out, "use ffi::{}::*;", cg.xcb_mod())?;
        for imp in imports.iter() {
            writeln!(&mut self.out, "use ffi::{}::*;", imp)?;
        }
        writeln!(&mut self.out, "")?;

        Ok(())
    }

    pub fn epilogue(&mut self) -> io::Result<()> {
        Ok(())
    }

    pub fn event(&mut self, cg: &mut CodeGen, ev: &Event) -> io::Result<()> {
        match ev {
            Event::XidType(name) => {
                let typ = cg.rust_type_name(name);
                let ffi_typ = cg.ffi_type_name(name);
                cg.emit_type_alias(&mut self.out, &typ, &ffi_typ)?;
                cg.reg_type(typ);
            }
            _ => {}
        }

        Ok(())
    }
}
