use crate::codegen::CodeGen;
use crate::output::Output;
use crate::xcbgen::Event;
use std::io::{self, Cursor, Write};

#[derive(Debug)]
pub struct FfiXcbEmit {
    cg: CodeGen,
    out: Output,                 // direct output
    ext_fn_out: Cursor<Vec<u8>>, // all the extern functions are grouped
}

impl FfiXcbEmit {
    pub fn new(cg: CodeGen, out: Output) -> FfiXcbEmit {
        FfiXcbEmit {
            cg,
            out,
            ext_fn_out: Cursor::new(Vec::new()),
        }
    }

    pub fn prologue(&mut self, imports: &Vec<String>) -> io::Result<()> {
        writeln!(
            &mut self.out,
            "use libc::{{c_char, c_int, c_uint, c_void}};"
        )?;
        writeln!(&mut self.out, "use std;")?;
        writeln!(&mut self.out)?;
        writeln!(&mut self.out, "use ffi::base::*;")?;
        for imp in imports.iter() {
            writeln!(&mut self.out, "use ffi::{}::*;", imp)?;
        }
        writeln!(&mut self.out)?;
        Ok(())
    }

    pub fn epilogue(&mut self) -> io::Result<()> {
        // write out all the external functions
        writeln!(&mut self.out).unwrap();
        writeln!(&mut self.out, "extern {{")?;

        self.out.write(self.ext_fn_out.get_ref())?;

        writeln!(&mut self.out).unwrap();
        writeln!(&mut self.out, "}} // extern")?;
        Ok(())
    }

    pub fn event(&mut self, ev: &Event) -> io::Result<()> {
        match ev {
            Event::XidType(name) => {
                self.cg
                    .emit_type_alias(&mut self.out, &self.cg.ffi_type_name(name), "u32")?;

                self.emit_iterator(name)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn emit_iterator(&mut self, typ: &str) -> io::Result<()> {
        let out = &mut self.out;
        let typ = typ.to_ascii_lowercase();
        let it_typ = self.cg.ffi_iterator_name(&typ);
        let it_next = self.cg.ffi_iterator_next_fn_name(&typ);
        let it_end = self.cg.ffi_iterator_end_fn_name(&typ);

        writeln!(out)?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "#[derive(Debug)]")?;
        writeln!(out, "pub struct {} {{", &it_typ)?;
        writeln!(out, "    pub data:  *mut xcb_{}_t,", &typ)?;
        writeln!(out, "    pub rem:   c_int,")?;
        writeln!(out, "    pub index: c_int,")?;
        writeln!(out, "}}")?;

        let out = &mut self.ext_fn_out;
        writeln!(out).unwrap();
        writeln!(out, "fn {}(i: *mut {});", &it_next, &it_typ).unwrap();
        writeln!(
            out,
            "fn {}(i: *mut {}) -> xcb_generic_iterator_t;",
            &it_end, &it_typ
        )
        .unwrap();
        Ok(())
    }
}
