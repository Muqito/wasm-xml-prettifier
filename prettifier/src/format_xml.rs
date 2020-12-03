use quick_xml::{events::Event, Reader, Writer};
use std::io::Cursor;

const NEW_LINE: &[u8] = "\n".as_bytes();

#[derive(Debug)]
pub enum PrettifyError {
    QuickXMLError,
    Save,
}
impl From<quick_xml::Error> for PrettifyError {
    fn from(e: quick_xml::Error) -> PrettifyError {
        dbg!(e);
        PrettifyError::QuickXMLError
    }
}

pub fn prettify_xml(input: &str) -> Result<String, PrettifyError> {
    let mut reader = Reader::from_str(input);
    reader.trim_text(true);
    #[derive(Debug, Clone, Copy)]
    enum XMLState {
        Start,
        End,
        Text,
        Inline,
        Init,
        Unknown,
    }
    fn indent(size: usize) -> String {
        let two_spaces = "  ";
        two_spaces.repeat(size)
    }

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf: Vec<u8> = Vec::new();
    let mut state = XMLState::Init;
    let mut depth = 0;

    loop {
        let event = reader.read_event(&mut buf)?;
        state = match (state, &event) {
            (_, Event::Eof) => break,
            (_, Event::Comment(_)) => continue,
            (XMLState::Init, event @ Event::Decl(_)) => {
                writer.write_event(event)?;
                XMLState::Init
            }
            (_, event @ Event::Start(_)) => {
                writer.write(NEW_LINE)?;
                writer.write(indent(depth).as_bytes())?;
                writer.write_event(event)?;
                depth += 1;
                XMLState::Start
            }
            (XMLState::Start, event @ Event::Text(_)) => {
                writer.write_event(event)?;
                XMLState::Text
            }
            (XMLState::Text, event @ Event::End(_)) => {
                writer.write_event(event)?;
                depth -= 1;
                XMLState::End
            }
            (XMLState::End, event @ Event::End(_)) => {
                depth -= 1;
                writer.write(NEW_LINE)?;
                writer.write(indent(depth).as_bytes())?;
                writer.write_event(event)?;
                XMLState::End
            }
            (XMLState::Start, event @ Event::Empty(_)) => {
                writer.write(NEW_LINE)?;
                writer.write(indent(depth).as_bytes())?;
                writer.write_event(event)?;
                XMLState::Inline
            }
            (XMLState::Start, event @ Event::End(_)) => {
                depth -= 1;
                writer.write_event(event)?;
                XMLState::End
            }
            (XMLState::End, event @ Event::Empty(_)) => {
                writer.write(NEW_LINE)?;
                writer.write(indent(depth).as_bytes())?;
                writer.write_event(event)?;
                XMLState::Inline
            }
            (XMLState::Inline, event @ Event::End(_)) => {
                depth -= 1;
                writer.write(NEW_LINE)?;
                writer.write(indent(depth).as_bytes())?;
                writer.write_event(event)?;
                XMLState::End
            }
            (XMLState::Inline, event @ Event::Empty(_)) => {
                writer.write(NEW_LINE)?;
                writer.write(indent(depth).as_bytes())?;
                writer.write_event(event)?;
                XMLState::Inline
            }
            (_, event) => {
                writer.write_event(event)?;
                XMLState::Unknown
            }
        };
        buf.clear();
    }
    let str = writer.into_inner().into_inner();
    String::from_utf8(str).map_err(|_| PrettifyError::Save)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_xml() {
        let content = r#"
<a>
  <b>
    <c>1</c>
    <d>
      <e>2</e>
      <e>3</e>
      <e>4</e>
    </d>
  </b>
</a>"#;
        let result = prettify_xml(&content);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), content);
    }
    #[test]
    fn incomplete_xml() {
        let content = r#"test</test>"#;
        let result = prettify_xml(&content);
        assert!(result.is_err());
    }
}
