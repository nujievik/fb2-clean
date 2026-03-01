use crate::{Result, Tags};
use quick_xml::{Reader, Writer, events::Event};
use std::io::{BufRead, Write};

/// Removes specified tags from an XML.
pub fn remove_xml_tags<R, W>(
    src: &mut Reader<R>,
    dest: &mut Writer<W>,
    rm_tags: &Tags,
) -> Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut buf = Vec::<u8>::new();
    let mut rm_depth = 0usize;

    loop {
        match src.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag = e.name().into_inner();
                if rm_depth > 0 || rm_tags.contains(tag) {
                    rm_depth += 1;
                } else {
                    dest.write_event(Event::Start(e))?;
                }
            }
            Ok(Event::End(e)) => {
                if rm_depth > 0 {
                    rm_depth -= 1;
                } else {
                    dest.write_event(Event::End(e))?;
                }
            }
            Ok(Event::Empty(e)) => {
                let tag = e.name().into_inner();
                if rm_depth == 0 && !rm_tags.contains(tag) {
                    dest.write_event(Event::Empty(e))?;
                }
            }
            Ok(Event::Eof) => break,
            Ok(event) if rm_depth == 0 => dest.write_event(event)?,
            Ok(_) => (),
            Err(err) => return Err(Box::new(err)),
        }
        buf.clear();
    }

    Ok(())
}
