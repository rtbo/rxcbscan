use crate::naming::Naming;
use crate::output::Output;
use crate::xcbgen::Event;
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

    pub fn emit_proloque(&mut self) -> io::Result<()> {
        writeln!(
            &mut self.out,
            "use libc::{{c_char, c_int, c_uint, c_void}};"
        )?;
        writeln!(&mut self.out, "use std;")?;
        writeln!(&mut self.out, "")?;
        writeln!(&mut self.out, "use ffi::base::*;")?;
        Ok(())
    }

    pub fn event(&mut self, ev: &Event) -> io::Result<()> {
        match ev {
            Event::Import(dep) => {
                writeln!(&mut self.out, "use ffi::{}::*;", dep)?;
            },
            Event::XidType(name) => {
                writeln!(&mut self.out, "")?;
                writeln!(
                    &mut self.out,
                    "pub type {} = u32;",
                    self.naming.ffi_type(&name)
                )?;
            }
            _ => {}
        }
        Ok(())
    }
}
