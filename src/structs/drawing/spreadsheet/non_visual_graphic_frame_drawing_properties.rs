// xdr:cNvGraphicFramePr
use writer::driver::*;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct NonVisualGraphicFrameDrawingProperties {

}
impl NonVisualGraphicFrameDrawingProperties {

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        e:&BytesStart
    ) {

    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:cNvGraphicFramePr
        write_start_tag(writer, "xdr:cNvGraphicFramePr", vec![], true);
    }
}
