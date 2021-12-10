// xdr:wsDr
use super::OneCellAnchor;
use super::TwoCellAnchor;
use super::Picture;
use super::Shape;
use super::ConnectionShape;
use super::GraphicFrame;
use writer::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct WorksheetDrawing {
    one_cell_anchor_collection: Vec<OneCellAnchor>,
    two_cell_anchor_collection: Vec<TwoCellAnchor>,
}
impl WorksheetDrawing {
    pub fn get_one_cell_anchor_collection(&self) -> &Vec<OneCellAnchor> {
        &self.one_cell_anchor_collection
    }

    pub fn get_one_cell_anchor_collection_mut(&mut self) -> &mut Vec<OneCellAnchor> {
        &mut self.one_cell_anchor_collection
    }

    pub fn add_one_cell_anchor_collection(&mut self, value:OneCellAnchor) {
        self.one_cell_anchor_collection.push(value);
    }

    pub fn get_two_cell_anchor_collection(&self) -> &Vec<TwoCellAnchor> {
        &self.two_cell_anchor_collection
    }

    pub fn get_two_cell_anchor_collection_mut(&mut self) -> &mut Vec<TwoCellAnchor> {
        &mut self.two_cell_anchor_collection
    }

    pub fn add_two_cell_anchor_collection(&mut self, value:TwoCellAnchor) {
        self.two_cell_anchor_collection.push(value);
    }

    pub fn has_drawing_object(&self)-> bool
    {
        &self.two_cell_anchor_collection.len() > &0usize
    }

    pub fn get_graphic_frame_collection(&self)-> Vec<&GraphicFrame>
    {
        let mut result:Vec<&GraphicFrame> = Vec::new(); 
        for two_cell_anchor in &self.two_cell_anchor_collection {
            match two_cell_anchor.get_graphic_frame() {
                Some(v) => {
                    result.push(v);
                },
                None => {}
            }
        }
        result
    }

    pub fn get_graphic_frame_collection_mut(&mut self)-> Vec<&mut GraphicFrame>
    {
        let mut result:Vec<&mut GraphicFrame> = Vec::new(); 
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            match two_cell_anchor.get_graphic_frame_mut() {
                Some(v) => {
                    result.push(v);
                },
                None => {}
            }
        }
        result
    }

    pub fn get_shape_collection(&self)-> Vec<&Shape>
    {
        let mut result:Vec<&Shape> = Vec::new(); 
        for two_cell_anchor in &self.two_cell_anchor_collection {
            match two_cell_anchor.get_shape() {
                Some(v) => {
                    result.push(v);
                },
                None => {}
            }
        }
        result
    }

    pub fn get_shape_collection_mut(&mut self)-> Vec<&mut Shape>
    {
        let mut result:Vec<&mut Shape> = Vec::new(); 
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            match two_cell_anchor.get_shape_mut() {
                Some(v) => {
                    result.push(v);
                },
                None => {}
            }
        }
        result
    }

    pub fn get_connection_shape_collection(&self)-> Vec<&ConnectionShape>
    {
        let mut result:Vec<&ConnectionShape> = Vec::new(); 
        for two_cell_anchor in &self.two_cell_anchor_collection {
            match two_cell_anchor.get_connection_shape() {
                Some(v) => {
                    result.push(v);
                },
                None => {}
            }
        }
        result
    }

    pub fn get_connection_shape_collection_mut(&mut self)-> Vec<&mut ConnectionShape>
    {
        let mut result:Vec<&mut ConnectionShape> = Vec::new(); 
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            match two_cell_anchor.get_connection_shape_mut() {
                Some(v) => {
                    result.push(v);
                },
                None => {}
            }
        }
        result
    }

    pub fn get_picture_collection(&self)-> Vec<&Picture>
    {
        let mut result:Vec<&Picture> = Vec::new(); 
        for two_cell_anchor in &self.two_cell_anchor_collection {
            match two_cell_anchor.get_picture() {
                Some(v) => {
                    result.push(v);
                },
                None => {}
            }
        }
        result
    }

    pub fn get_picture_collection_mut(&mut self)-> Vec<&mut Picture>
    {
        let mut result:Vec<&mut Picture> = Vec::new(); 
        for two_cell_anchor in &mut self.two_cell_anchor_collection {
            match two_cell_anchor.get_picture_mut() {
                Some(v) => {
                    result.push(v);
                },
                None => {}
            }
        }
        result
    }

    pub(crate) fn set_attributes<R: std::io::BufRead, A: std::io::Read + std::io::Seek>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        target: &str,
    ) {
        let mut is_alternate_content = false;
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"xdr:oneCellAnchor" => {
                            if is_alternate_content {
                                continue;
                            }
                            let mut obj = OneCellAnchor::default();
                            obj.set_attributes(reader, e);
                            &mut self.add_one_cell_anchor_collection(obj);
                        },
                        b"xdr:twoCellAnchor" => {
                            if is_alternate_content {
                                continue;
                            }
                            let mut obj = TwoCellAnchor::default();
                            obj.set_attributes(reader, e, arv, target);
                            if obj.is_support() {
                                &mut self.add_two_cell_anchor_collection(obj);
                            }
                        },
                        b"mc:AlternateContent" => {
                            is_alternate_content = true;
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"mc:AlternateContent" => {
                            is_alternate_content = false;
                        },
                        b"xdr:wsDr" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:wsDr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:wsDr
        write_start_tag(writer, "xdr:wsDr", vec![
            ("xmlns:xdr", "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing"),
            ("xmlns:a", "http://schemas.openxmlformats.org/drawingml/2006/main"),
        ], false);

        // xdr:oneCellAnchor
        for one_cell_anchor in &self.one_cell_anchor_collection {
            one_cell_anchor.write_to(writer);
        }

        // xdr:twoCellAnchor
        let mut r_id = 1;
        for two_cell_anchor in &self.two_cell_anchor_collection {
            two_cell_anchor.write_to(writer, &mut r_id);
        }
        
        write_end_tag(writer, "xdr:wsDr");
    }
}
