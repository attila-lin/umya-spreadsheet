// c:cat
use super::NumberReference;
use super::StringLiteral;
use super::StringReference;
use crate::reader::driver::*;
use crate::structs::Spreadsheet;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct CategoryAxisData {
    string_reference: Option<StringReference>,
    string_literal: Option<StringLiteral>,
    number_reference: Option<NumberReference>,
}

impl CategoryAxisData {
    pub fn get_string_reference(&self) -> Option<&StringReference> {
        self.string_reference.as_ref()
    }

    pub fn get_string_reference_mut(&mut self) -> Option<&mut StringReference> {
        self.string_reference.as_mut()
    }

    pub fn set_string_reference(&mut self, value: StringReference) -> &mut Self {
        self.string_reference = Some(value);
        self
    }

    pub fn remove_string_reference(&mut self) -> &mut Self {
        self.string_reference = None;
        self
    }

    pub fn get_string_literal(&self) -> Option<&StringLiteral> {
        self.string_literal.as_ref()
    }

    pub fn get_string_literal_mut(&mut self) -> Option<&mut StringLiteral> {
        self.string_literal.as_mut()
    }

    pub fn set_string_literal(&mut self, value: StringLiteral) -> &mut Self {
        self.string_literal = Some(value);
        self
    }

    pub fn remove_string_literal(&mut self) -> &mut Self {
        self.string_literal = None;
        self
    }

    pub fn get_number_reference(&self) -> Option<&NumberReference> {
        self.number_reference.as_ref()
    }

    pub fn get_number_reference_mut(&mut self) -> Option<&mut NumberReference> {
        self.number_reference.as_mut()
    }

    pub fn set_number_reference(&mut self, value: NumberReference) -> &mut Self {
        self.number_reference = Some(value);
        self
    }

    pub fn remove_number_reference(&mut self) -> &mut Self {
        self.number_reference = None;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"c:strRef" => {
                        let mut obj = StringReference::default();
                        obj.set_attributes(reader, e);
                        self.set_string_reference(obj);
                    }
                    b"c:strLit" => {
                        let mut obj = StringLiteral::default();
                        obj.set_attributes(reader, e);
                        self.set_string_literal(obj);
                    }
                    b"c:numRef" => {
                        let mut obj = NumberReference::default();
                        obj.set_attributes(reader, e);
                        self.set_number_reference(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:cat" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:cat")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:cat
        write_start_tag(writer, "c:cat", vec![], false);

        // c:strRef
        if let Some(v) = &self.string_reference {
            v.write_to(writer, spreadsheet);
        }

        // c:strLit
        if let Some(v) = &self.string_literal {
            v.write_to(writer);
        }

        // c:numRef
        if let Some(v) = &self.number_reference {
            v.write_to(writer, spreadsheet);
        }

        write_end_tag(writer, "c:cat");
    }
}
