use crate::output::Output;
use crate::xcbgen::XcbEmit;
use std::io::{self};

pub struct RustXcbEmit {
    out: Output,
}

impl RustXcbEmit {
    pub fn new(out: Output) -> RustXcbEmit {
        RustXcbEmit { out }
    }
}

impl XcbEmit for RustXcbEmit {
    fn emit_xidtype(&mut self, name: &str) -> io::Result<()> {
        Ok(())
    }
}
