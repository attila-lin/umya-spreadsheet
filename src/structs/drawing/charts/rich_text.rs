// c:rich
use super::super::BodyProperties;
use super::super::ListStyle;
use super::super::Paragraph;
use crate::drawing::Run;
use crate::writer::driver::*;
use crate::xml_read_loop;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct RichText {
    body_properties: BodyProperties,
    list_style: ListStyle,
    paragraph: ThinVec<Paragraph>,
}

impl RichText {
    #[inline]
    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.paragraph.clear();
        let mut paragraph = Paragraph::default();
        let mut run = Run::default();
        run.set_text(value);
        paragraph.add_run(run);
        self.add_paragraph(paragraph);
        self
    }

    pub fn get_body_properties(&self) -> &BodyProperties {
        &self.body_properties
    }

    pub fn get_body_properties_mut(&mut self) -> &mut BodyProperties {
        &mut self.body_properties
    }

    pub fn set_body_properties(&mut self, value: BodyProperties) {
        self.body_properties = value;
    }

    pub fn get_list_style(&self) -> &ListStyle {
        &self.list_style
    }

    pub fn get_list_style_mut(&mut self) -> &mut ListStyle {
        &mut self.list_style
    }

    pub fn set_list_style(&mut self, value: ListStyle) {
        self.list_style = value;
    }

    pub fn get_paragraph(&self) -> &[Paragraph] {
        &self.paragraph
    }

    pub fn get_paragraph_mut(&mut self) -> &mut ThinVec<Paragraph> {
        &mut self.paragraph
    }

    pub fn add_paragraph(&mut self, value: Paragraph) {
        self.paragraph.push(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => match e.name().0 {
                b"a:p" => {
                    let mut paragraph = Paragraph::default();
                    paragraph.set_attributes(reader, e);
                    self.add_paragraph(paragraph);
                }
                b"a:bodyPr" => {
                    let mut body_properties = BodyProperties::default();
                    body_properties.set_attributes(reader, e, false);
                    self.set_body_properties(body_properties);
                }
                _ => (),
            },
            Event::Empty(ref e) => {
                if e.name().0 == b"a:bodyPr" {
                    let mut body_properties = BodyProperties::default();
                    body_properties.set_attributes(reader, e, true);
                    self.set_body_properties(body_properties);
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:rich" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:rich"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:rich
        write_start_tag(writer, "c:rich", vec![], false);

        // a:bodyPr
        self.body_properties.write_to(writer);

        // a:lstStyle
        write_start_tag(writer, "a:lstStyle", vec![], true);

        for content in &self.paragraph {
            content.write_to(writer);
        }

        write_end_tag(writer, "c:rich");
    }
}
