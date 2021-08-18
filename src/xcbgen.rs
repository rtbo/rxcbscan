use crate::codegen::CodeGen;
use crate::ffi::FfiXcbEmit;
use crate::output::Output;
use crate::rust::RustXcbEmit;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event as XmlEv;
use quick_xml::Reader as XmlReader;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::{self, Utf8Error};
// pub struct TypAnnot {
//     name: String,
//     borrow: bool,
//     mutable: bool,
// }

// pub struct Field {
//     name: String,
//     typ: TypAnnot,
// }

// pub struct EnumItem {
//     name: String,
//     value: u32,
// }

// pub struct Enum {
//     name: String,
//     items: Vec<EnumItem>,
// }

#[derive(Debug)]
pub enum XcbGenError {
    IO(io::Error),
    Xml(quick_xml::Error),
    Utf8(Utf8Error),
    Parse(String),
}

impl From<io::Error> for XcbGenError {
    fn from(err: io::Error) -> Self {
        XcbGenError::IO(err)
    }
}

impl From<Utf8Error> for XcbGenError {
    fn from(err: Utf8Error) -> Self {
        XcbGenError::Utf8(err)
    }
}

impl From<quick_xml::Error> for XcbGenError {
    fn from(err: quick_xml::Error) -> Self {
        match err {
            quick_xml::Error::Io(e) => XcbGenError::IO(e),
            quick_xml::Error::Utf8(e) => XcbGenError::Utf8(e),
            e => XcbGenError::Xml(e),
        }
    }
}

pub type XcbGenResult<T> = Result<T, XcbGenError>;

#[derive(Debug)]
pub struct XcbGen {
    ffi: FfiXcbEmit,
    rust: RustXcbEmit,
    xcb_mod: String,
}

impl XcbGen {
    pub fn new(xcb_mod: &str, ffi: Output, rust: Output) -> XcbGen {
        let codegen = CodeGen::new(xcb_mod);

        let ffi = FfiXcbEmit::new(codegen.clone(), ffi);
        let rust = RustXcbEmit::new(codegen, rust);
        XcbGen {
            ffi,
            rust,
            xcb_mod: xcb_mod.into(),
        }
    }

    pub fn xcb_gen(mut self, xml_file: &Path) -> XcbGenResult<()> {
        let mut xml = XmlReader::from_file(xml_file).unwrap();
        xml.trim_text(true);

        let mut parser = XcbParser {
            xml,
            buf: Vec::with_capacity(8 * 1024),
        };

        let mut imports = Vec::new();
        let mut fst: Option<XcbGenResult<Event>> = None;

        for e in &mut parser {
            match e? {
                Event::Ignore => {}
                Event::Import(imp) => imports.push(imp),
                ev => {
                    fst = Some(Ok(ev));
                    break;
                }
            }
        }

        self.ffi.prologue(&imports)?;
        self.rust.prologue(&imports)?;

        for ev in fst.into_iter().chain(&mut parser) {
            match ev {
                Ok(ev) => {
                    self.ffi.event(&ev)?;
                    self.rust.event(&ev)?;
                }
                Err(ev) => Err(ev)?,
            }
        }

        self.ffi.epilogue()?;
        self.rust.epilogue()?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Event {
    Import(String),
    XidType(String),
    Ignore,
}

struct XcbParser<B: BufRead> {
    xml: XmlReader<B>,
    buf: Vec<u8>,
}

impl<B: BufRead> XcbParser<B> {
    fn expect_text(&mut self) -> XcbGenResult<String> {
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Text(e)) => Ok(e.unescape_and_decode(&self.xml)?),
            Ok(ev) => Err(XcbGenError::Parse(format!("expected text, found {:?}", ev))),
            Err(e) => Err(e)?,
        }
    }

    fn expect_close_tag(&mut self, tag: &[u8]) -> XcbGenResult<()> {
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::End(e)) => {
                if e.name() == tag {
                    Ok(())
                } else {
                    Err(XcbGenError::Parse(format!(
                        "expected </{}>, got </{}>",
                        str::from_utf8(tag).unwrap(),
                        str::from_utf8(e.name())?
                    )))
                }
            }
            Ok(ev) => Err(XcbGenError::Parse(format!(
                "expected </{}>, found {:?}",
                str::from_utf8(tag).unwrap(),
                ev
            ))),
            Err(e) => Err(e)?,
        }
    }

    fn parse_import(&mut self) -> XcbGenResult<String> {
        let imp = self.expect_text()?;
        self.expect_close_tag(b"import")?;
        Ok(imp)
    }
}

impl<B: BufRead> Iterator for &mut XcbParser<B> {
    type Item = XcbGenResult<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Empty(ref e) | XmlEv::Start(ref e)) => match e.name() {
                b"import" => Some(self.parse_import().map(|s| Event::Import(s))),
                b"xidtype" => {
                    let attrs = e.attributes();
                    let typres = expect_attribute(attrs, b"name");
                    Some(typres.map(|v| Event::XidType(v)))
                }
                _ => Some(Ok(Event::Ignore)),
            },

            Ok(XmlEv::Eof) => None,

            _ => Some(Ok(Event::Ignore)),
        }
    }
}

fn expect_attribute(attrs: Attributes, name: &[u8]) -> XcbGenResult<String> {
    for attr in attrs {
        match attr {
            Ok(attr) => {
                if attr.key == name {
                    let val = attr.unescaped_value()?;
                    return Ok(str::from_utf8(&val)?.to_string());
                }
            }
            Err(err) => Err(err)?,
        }
    }
    Err(XcbGenError::Parse(format!(
        "could not find expected attribute: {}",
        str::from_utf8(name).unwrap()
    )))
}
