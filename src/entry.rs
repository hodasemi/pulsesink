use command;
use constants::*;

#[derive(Clone)]
pub struct Entry {
    name: String,
}

fn crop_letters(s: &mut String, pos: usize) {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => {
            s.drain(0..pos);
        }
        None => {
            s.clear();
        }
    }
}

fn remove_prefix(string: &String, prefix: &str) -> String {
    let mut tmp_line = string.clone();
    crop_letters(&mut tmp_line, prefix.len());
    tmp_line.replace("\"", "")
}

fn filter(terms: &[&str], lines: &Vec<String>) -> Vec<String> {
    let mut filtered_lines = Vec::new();

    for line in lines {
        for term in terms {
            if line.as_str().contains(term) {
                filtered_lines.push(line.clone());
            }
        }
    }

    filtered_lines
}

impl Entry {
    fn create_entries(infos: &Vec<String>) -> Vec<Entry> {
        let mut entries = Vec::new();

        let mut current_entry: Option<Entry> = None;

        for line in infos {
            if line.as_str().contains(NAME) {
                current_entry = Some(Entry::new());
            } else if line.as_str().contains(PRODUCT_NAME) {
                if let Some(ref mut entry) = current_entry {
                    entry.name = remove_prefix(line, PRODUCT_NAME);
                    entries.push(entry.clone());
                }

                current_entry = None;
            } else if line.as_str().contains(PRODUCT_DESCRIPTION) {
                if let Some(ref mut entry) = current_entry {
                    entry.name = remove_prefix(line, PRODUCT_DESCRIPTION);
                    entries.push(entry.clone());
                }

                current_entry = None;
            }
        }

        entries
    }

    fn new() -> Entry {
        Entry {
            name: String::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

pub fn get_all_sinks() -> Result<Vec<Entry>, String> {
    let lines = match command::bash("pacmd", &["list-sinks"]) {
        Ok(lines) => lines,
        Err(err) => return Err(err),
    };

    let filtered = filter(&[NAME, PRODUCT_NAME, PRODUCT_DESCRIPTION], &lines);

    Ok(Entry::create_entries(&filtered))
}
