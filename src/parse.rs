use quick_xml::events::attributes::{Attribute, Attributes};
use quick_xml::events::{BytesStart, Event as XmlEv};
use quick_xml::Reader as XmlReader;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::result;
use std::str::{self, FromStr, Utf8Error};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Xml(quick_xml::Error),
    Utf8(Utf8Error),
    Parse(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        match err {
            quick_xml::Error::Io(e) => Error::IO(e),
            quick_xml::Error::Utf8(e) => Error::Utf8(e),
            e => Error::Xml(e),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct DocField {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum Expr<T>
where
    T: FromStr,
    T: Clone,
{
    FieldRef(String),
    ParamRef(String),
    Value(T),
    Op(String, Box<Expr<T>>, Box<Expr<T>>),
    Unop(String, Box<Expr<T>>),
    Popcount(Box<Expr<T>>),
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub brief: String,
    pub text: String,
    pub fields: Vec<DocField>,
}

#[derive(Debug, Clone)]
pub struct EnumItem {
    pub id: String,
    pub name: String,
    pub value: u32,
}

#[derive(Debug, Clone)]
pub enum StructField {
    Field {
        id: String,
        name: String,
        typ: String,
        enu: Option<String>,
    },
    List {
        id: String,
        name: String,
        typ: String,
        len_expr: Expr<usize>,
    },
    Pad(usize),
    AlignPad(usize),
}

#[derive(Debug, Clone)]
pub enum Event {
    Import(String),
    Typedef {
        oldname: String,
        newname: String,
    },
    XidType(String),
    Enum {
        name: String,
        bitset: bool,
        items: Vec<EnumItem>,
        doc: Option<Doc>,
    },
    Struct {
        name: String,
        fields: Vec<StructField>,
        doc: Option<Doc>,
    },
    Ignore,
}

pub struct Parser<B: BufRead> {
    xml: XmlReader<B>,
    buf: Vec<u8>,
}

impl Parser<BufReader<File>> {
    pub fn from_file(xml_file: &Path) -> Self {
        let mut xml = XmlReader::from_file(xml_file).unwrap();
        xml.trim_text(true);

        Parser {
            xml,
            buf: Vec::with_capacity(8 * 1024),
        }
    }
}

impl<B: BufRead> Parser<B> {
    fn expect_text(&mut self) -> Result<String> {
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Text(e) | XmlEv::CData(e)) => {
                let txt = e.unescaped()?;
                Ok(str::from_utf8(&txt)?.into())
            }
            Ok(ev) => Err(Error::Parse(format!("expected text, found {:?}", ev))),
            Err(e) => Err(e)?,
        }
    }

    fn expect_text_trim(&mut self) -> Result<String> {
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Text(e) | XmlEv::CData(e)) => {
                let txt = e.unescaped()?;
                Ok(str::from_utf8(&txt)?.trim().into())
            }
            Ok(ev) => Err(Error::Parse(format!("expected text, found {:?}", ev))),
            Err(e) => Err(e)?,
        }
    }

    fn expect_start(&mut self) -> Result<Vec<u8>> {
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Start(e) | XmlEv::Empty(e)) => Ok(Vec::from(e.name())),
            Ok(ev) => Err(Error::Parse(format!("expected start tag, found {:?}", ev))),
            Err(e) => Err(e)?,
        }
    }

    fn expect_close_tag(&mut self, tag: &[u8]) -> Result<()> {
        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::End(e)) => {
                    if e.name() == tag {
                        return Ok(());
                    } else {
                        return Err(Error::Parse(format!(
                            "expected </{}>, got </{}>",
                            str::from_utf8(tag).unwrap(),
                            str::from_utf8(e.name())?
                        )));
                    }
                }
                Ok(XmlEv::Comment(_)) => {}
                Ok(ev) => {
                    return Err(Error::Parse(format!(
                        "expected </{}>, found {:?}",
                        str::from_utf8(tag).unwrap(),
                        ev
                    )))
                }
                Err(e) => Err(e)?,
            }
        }
    }

    fn expect_text_element(&mut self) -> Result<(Vec<u8>, String)> {
        let tag = self.expect_start()?;
        let txt = self.expect_text()?;
        self.expect_close_tag(&tag)?;
        Ok((tag, txt))
    }

    // fn expect_start_tag(&mut self, tag: &[u8]) -> Result<()> {
    //     match self.xml.read_event(&mut self.buf) {
    //         Ok(XmlEv::Start(e) | XmlEv::Empty(e)) => {
    //             if e.name() == tag {
    //                 Ok(())
    //             } else {
    //                 Err(Error::Parse(format!(
    //                     "expected <{}>, got <{}>",
    //                     str::from_utf8(tag).unwrap(),
    //                     str::from_utf8(e.name())?
    //                 )))
    //             }
    //         }
    //         Ok(ev) => Err(Error::Parse(format!(
    //             "expected <{}>, found {:?}",
    //             str::from_utf8(tag).unwrap(),
    //             ev
    //         ))),
    //         Err(e) => Err(e)?,
    //     }
    // }

    // fn expect_text_element_with_tag(&mut self, tag: &[u8]) -> Result<String> {
    //     self.expect_start_tag(tag)?;
    //     let txt = self.expect_text()?;
    //     self.expect_close_tag(tag)?;
    //     Ok(txt)
    // }

    fn parse_doc(&mut self) -> Result<Doc> {
        let mut brief = String::new();
        let mut text = String::new();
        let mut fields = Vec::new();

        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::Start(ref e)) => match e.name() {
                    b"brief" => {
                        brief = self.expect_text_trim()?;
                    }
                    b"field" => {
                        let name = expect_attribute(e.attributes(), b"name")?;
                        let text = self.expect_text_trim()?;
                        fields.push(DocField { name, text });
                    }
                    _ => {}
                },
                Ok(XmlEv::Text(txt) | XmlEv::CData(txt)) => {
                    let txt = txt.unescaped()?;
                    text.push_str(str::from_utf8(&txt)?);
                }
                Ok(XmlEv::End(ref e)) => {
                    if e.name() == b"doc" {
                        break;
                    }
                }
                Ok(_) => {}
                Err(err) => Err(err)?,
            }
        }

        Ok(Doc {
            brief,
            text: text.trim().into(),
            fields,
        })
    }

    fn parse_expr<T>(&mut self) -> Result<Expr<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
        T: Clone,
    {
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Start(ref e)) => match e.name() {
                b"fieldref" => {
                    let fr = self.expect_text_trim()?;
                    self.expect_close_tag(b"fieldref")?;
                    Ok(Expr::FieldRef(fr))
                }
                b"paramref" => {
                    let pr = self.expect_text_trim()?;
                    self.expect_close_tag(b"paramref")?;
                    Ok(Expr::ParamRef(pr))
                }
                b"value" => {
                    let val = self.expect_text_trim()?;
                    self.expect_close_tag(b"value")?;
                    let val: T = val.parse().map_err(|e| {
                        Error::Parse(format!("could not parse expr <value> tag: {}", e))
                    })?;
                    Ok(Expr::Value(val))
                }
                b"op" => {
                    let op = expect_attribute(e.attributes(), b"op")?;
                    let lhs = self.parse_expr::<T>()?;
                    let rhs = self.parse_expr::<T>()?;
                    self.expect_close_tag(b"op")?;
                    Ok(Expr::Op(op, Box::new(lhs), Box::new(rhs)))
                }
                b"unop" => {
                    let unop = expect_attribute(e.attributes(), b"op")?;
                    let expr = self.parse_expr::<T>()?;
                    self.expect_close_tag(b"unop")?;
                    Ok(Expr::Unop(unop, Box::new(expr)))
                }
                b"popcount" => {
                    let expr = self.parse_expr::<T>()?;
                    self.expect_close_tag(b"popcount")?;
                    Ok(Expr::Popcount(Box::new(expr)))
                }
                tag => Err(Error::Parse(format!(
                    "Unexpected <{}> in expression",
                    str::from_utf8(tag)?
                ))),
            },
            Ok(XmlEv::Comment(_)) => self.parse_expr::<T>(), // in case of comment, we just parse the next one
            Ok(ev) => Err(Error::Parse(format!(
                "Unexpected XML while parsing expression: {:?}",
                ev
            ))),
            Err(err) => Err(err.into()),
        }
    }

    fn parse_import(&mut self) -> Result<String> {
        let imp = self.expect_text()?;
        self.expect_close_tag(b"import")?;
        Ok(imp)
    }

    fn parse_enum(
        &mut self,
        start: BytesStart,
    ) -> Result<(String, bool, Vec<EnumItem>, Option<Doc>)> {
        let name = expect_attribute(start.attributes(), b"name")?;
        let mut items = Vec::new();
        let mut bitset = false;
        let mut doc = None;

        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::Empty(ref e) | XmlEv::Start(ref e)) => match e.name() {
                    b"item" => {
                        let name = expect_attribute(e.attributes(), b"name")?;
                        let (tag, value) = self.expect_text_element()?;
                        if tag != b"bit" && tag != b"value" {
                            return Err(Error::Parse(format!(
                                "expected <bit> or <value> for enum {}, got <{}>",
                                name,
                                str::from_utf8(&tag)?
                            )));
                        }
                        let value: u32 = value.parse().map_err(|e| {
                            Error::Parse(format!(
                                "failed to parse {} of enum {}: {}",
                                str::from_utf8(&tag).unwrap(),
                                name,
                                e
                            ))
                        })?;
                        let is_bit = tag == b"bit";
                        if is_bit {
                            bitset = true;
                        }
                        let value = if is_bit { 1 << value } else { value };
                        items.push(EnumItem {
                            id: name.clone(),
                            name,
                            value,
                        });
                    }
                    b"doc" => {
                        doc = Some(self.parse_doc()?);
                    }
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected tag in enum: {}",
                            str::from_utf8(tag)?
                        )));
                    }
                },
                Ok(XmlEv::End(ref e)) => match e.name() {
                    b"enum" => break,
                    b"item" => continue,
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected </{}> in enum {}",
                            str::from_utf8(tag)?,
                            name,
                        )))
                    }
                },
                Ok(XmlEv::Comment(_)) => {}
                Ok(ev) => {
                    return Err(Error::Parse(format!("unexpected XML in enum: {:?}", ev)));
                }
                Err(err) => Err(err)?,
            }
        }

        Ok((name, bitset, items, doc))
    }

    fn parse_struct(&mut self) -> Result<(Vec<StructField>, Option<Doc>)> {
        let mut fields = Vec::new();
        let mut doc = None;
        let mut had_list = false;

        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::Start(ref e) | XmlEv::Empty(ref e)) => match e.name() {
                    b"field" => {
                        let names: [&[u8]; 3] = [b"type", b"name", b"enum"];
                        let mut vals: [Option<String>; 3] = [None, None, None];
                        get_attributes(e.attributes(), &names, &mut vals)?;
                        let [typ, nam, enu] = vals;

                        if let (Some(typ), Some(name)) = (typ, nam) {
                            fields.push(StructField::Field {
                                id: name.clone(),
                                name: name,
                                typ: typ,
                                enu: enu,
                            })
                        } else {
                            return Err(Error::Parse(
                                "struct field without type and/or name".into(),
                            ));
                        }
                    }
                    b"pad" => {
                        let names: [&[u8]; 2] = [b"bytes", b"align"];
                        let mut vals: [Option<String>; 2] = [None, None];
                        get_attributes(e.attributes(), &names, &mut vals)?;
                        let [bytes, align] = vals;
                        if bytes.is_some() && align.is_some() {
                            return Err(Error::Parse(
                                "<pad> with both align and bytes attr".into(),
                            ));
                        }
                        if let Some(bytes) = bytes {
                            let val: usize = bytes.parse().map_err(|e| {
                                Error::Parse(format!("failed to parse pad bytes of struct: {}", e))
                            })?;
                            fields.push(StructField::Pad(val));
                        } else if let Some(align) = align {
                            if !had_list {
                                return Err(Error::Parse(
                                    "alignment pad only expected after list field".into(),
                                ));
                            }
                            let val: usize = align.parse().map_err(|e| {
                                Error::Parse(format!("failed to parse pad bytes of struct: {}", e))
                            })?;
                            fields.push(StructField::AlignPad(val));
                        } else {
                            return Err(Error::Parse(
                                "<pad> with neither align and bytes attr".into(),
                            ));
                        }
                    }
                    b"list" => {
                        let names: [&[u8]; 2] = [b"type", b"name"];
                        let mut vals: [Option<String>; 2] = [None, None];
                        get_attributes(e.attributes(), &names, &mut vals)?;
                        let [typ, nam] = vals;
                        if let (Some(typ), Some(name)) = (typ, nam) {
                            let len_expr = self.parse_expr::<usize>()?;
                            fields.push(StructField::List {
                                id: name.clone(),
                                name,
                                typ,
                                len_expr,
                            });
                            had_list = true;
                        } else {
                            return Err(Error::Parse(
                                "<list> tag without type and/or name attribute".into(),
                            ));
                        }
                    }
                    b"doc" => {
                        doc = Some(self.parse_doc()?);
                    }
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected <{}> in struct",
                            str::from_utf8(tag)?
                        )))
                    }
                },
                Ok(XmlEv::End(ref e)) => match e.name() {
                    b"struct" => break,
                    _ => {}
                },
                Ok(XmlEv::Comment(_)) => {}
                Ok(ev) => {
                    return Err(Error::Parse(format!("unexpected XML in struct: {:?}", ev)));
                }
                Err(err) => return Err(err.into()),
            }
        }

        Ok((fields, doc))
    }
}

impl<B: BufRead> Iterator for &mut Parser<B> {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Empty(ref e) | XmlEv::Start(ref e)) => match e.name() {
                b"import" => Some(self.parse_import().map(|s| Event::Import(s))),
                b"typedef" => {
                    let mut oldname: Option<String> = None;
                    let mut newname: Option<String> = None;
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            let valres = attr_value(&attr);
                            if let Err(err) = valres {
                                return Some(Err(err));
                            }
                            match attr.key {
                                b"oldname" => {
                                    oldname = Some(valres.unwrap());
                                }
                                b"newname" => {
                                    newname = Some(valres.unwrap());
                                }
                                _ => {}
                            }
                        }
                    }
                    match (oldname, newname) {
                        (Some(oldname), Some(newname)) => {
                            Some(Ok(Event::Typedef { oldname, newname }))
                        }
                        _ => Some(Err(Error::Parse(
                            "typedef without newname and/or oldname".into(),
                        ))),
                    }
                }
                b"xidtype" => {
                    let attrs = e.attributes();
                    let typres = expect_attribute(attrs, b"name");
                    Some(typres.map(|v| Event::XidType(v)))
                }
                b"enum" => {
                    let start = e.to_owned();
                    let enumres = self.parse_enum(start);
                    Some(enumres.map(|(name, bitset, items, doc)| Event::Enum {
                        name,
                        bitset,
                        items,
                        doc,
                    }))
                }
                b"struct" => {
                    let nameres = expect_attribute(e.attributes(), b"name");
                    if let Err(err) = nameres {
                        return Some(Err(err));
                    }
                    let structres = self.parse_struct();
                    Some(structres.map(|(fields, doc)| Event::Struct {
                        name: nameres.unwrap(),
                        fields,
                        doc,
                    }))
                }
                _ => Some(Ok(Event::Ignore)),
            },

            Ok(XmlEv::Eof) => None,

            _ => Some(Ok(Event::Ignore)),
        }
    }
}

fn attr_value(attr: &Attribute) -> Result<String> {
    let val = attr.unescaped_value()?;
    Ok(str::from_utf8(&val)?.into())
}

fn expect_attribute(attrs: Attributes, name: &[u8]) -> Result<String> {
    for attr in attrs {
        match attr {
            Ok(attr) => {
                if attr.key == name {
                    return attr_value(&attr);
                }
            }
            Err(err) => Err(err)?,
        }
    }
    Err(Error::Parse(format!(
        "could not find expected attribute: {}",
        str::from_utf8(name).unwrap()
    )))
}

fn get_attributes(attrs: Attributes, names: &[&[u8]], output: &mut [Option<String>]) -> Result<()> {
    assert_eq!(names.len(), output.len());
    for attr in attrs {
        match attr {
            Ok(attr) => {
                for (i, nam) in names.iter().enumerate() {
                    if attr.key == *nam {
                        output[i] = Some(attr_value(&attr)?);
                    }
                }
            }
            Err(err) => return Err(err.into()),
        }
    }
    Ok(())
}
