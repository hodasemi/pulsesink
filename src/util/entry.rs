use constants::*;

pub struct Entry {
    id: String,
    name: String,
}

/*
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

fn remove_prefix(string: &String, prefix: &str, delimiters: &[&str]) -> String {
    let mut tmp_line = string.clone();
    crop_letters(&mut tmp_line, prefix.len());
    for delimiter in delimiters {
        tmp_line = tmp_line.replace(delimiter, "");
    }

    tmp_line.trim().to_string()
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
*/

/*
fn extract_id(name: &str) -> String {}
*/

impl Entry {
    /*
    fn create_entries(infos: &Vec<String>) -> Vec<Entry> {
        let mut entries = Vec::new();

        let mut current_entry: Option<Entry> = None;

        for line in infos {
            if line.as_str().contains(NAME) {
                current_entry = Some(Entry::new());
                if let Some(ref mut entry) = current_entry {
                    entry.id = remove_prefix(line, NAME, &["<", ">"]);
                }
            } else if line.as_str().contains(PRODUCT_NAME) {
                if let Some(ref mut entry) = current_entry {
                    entry.name = remove_prefix(line, PRODUCT_NAME, &["\""]);
                    entries.push(entry.clone());
                }

                current_entry = None;
            } else if line.as_str().contains(PRODUCT_DESCRIPTION) {
                if let Some(ref mut entry) = current_entry {
                    entry.name = remove_prefix(line, PRODUCT_DESCRIPTION, &["\""]);
                    entries.push(entry.clone());
                }

                current_entry = None;
            }
        }

        entries
    }
    */

    pub fn new(id: String, name: String) -> Entry {
        Entry { id: id, name: name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn find_id_by_name<'a>(entries: &'a Vec<Entry>, name: &str) -> Option<&'a String> {
        for entry in entries {
            if entry.name == name {
                return Some(entry.id());
            }
        }

        None
    }
}

/*
pub fn get_all_sinks() -> Result<Vec<Entry>, String> {
    let lines = match command::bash("pacmd", &["list-sinks"]) {
        Ok(lines) => lines,
        Err(err) => return Err(err),
    };

    let filtered = filter(&[NAME, PRODUCT_NAME, PRODUCT_DESCRIPTION], &lines);

    Ok(Entry::create_entries(&filtered))
}
*/
