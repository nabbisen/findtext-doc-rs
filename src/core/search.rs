use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;
use zip::ZipArchive;

const MAX_TEXT_ACCUMULATOR_LEN: usize = 100;

/// search main handler
pub fn handle(keyword: &str, filepath: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;

        if file.name() != "word/document.xml" {
            continue;
        }

        let reader = BufReader::new(file);
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);

        // Event::Text gives text portions as BytesText. They are often fewer than keyword chars
        let mut text_accumulator = String::new();
        let max_text_accumulator_len = if MAX_TEXT_ACCUMULATOR_LEN < keyword.len() - 1 {
            keyword.len() - 1
        } else {
            MAX_TEXT_ACCUMULATOR_LEN
        };

        let mut buffer = Vec::new();
        loop {
            match xml_reader.read_event_into(&mut buffer) {
                Ok(Event::Text(e)) => {
                    let text = e.as_ref();
                    text_accumulator.push_str(&String::from_utf8_lossy(text));

                    if text_accumulator.contains(keyword) {
                        return Ok(true);
                    }

                    // optimize accumulator
                    text_accumulator = text_accumulator
                        .chars()
                        .rev()
                        .take(max_text_accumulator_len)
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect()
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    eprintln!("Error reading XML: {:?}", e);
                    break;
                }
                _ => {}
            }
            buffer.clear();
        }
    }

    return Ok(false);
}
