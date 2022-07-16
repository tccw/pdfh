use std::{collections::{HashSet}, path::{PathBuf}, fmt::Error};
use lopdf::{Document};

/// Deletes the pages listed in --pages, or deletes every --every page in a PDF
/// 
/// * `infile` - a PathBuf of a single file
/// * `outfile` - a PathBuf representing the location to save the output file to
/// * `pages` - a list of page numbers to delete
/// * `every` - an integer 
/// * `negate` - negates/inverts the --page or --every selection, instead keeping only those pages listed
/// * `compress` - a boolean flag to compress the outfile before saving
/// 
pub fn delete(infile: PathBuf, 
    outfile: Option<PathBuf>, 
    pages: Option<Vec<u32>>,
    every: Option<u32>,
    negate: bool,
    compress: bool) {

    let mut doc: Document = load_pdf(&infile);

    delete_pages(&mut doc, pages, every, negate);

    if compress { doc.compress() }

    let mut result = None;
    match outfile {
        Some(f) => {
            result = Some(doc.save(f));
        }
        None => {
            result = Some(doc.save(infile));
        }
    }

    match result {
        Some(res) => {
            match res {
                Ok(_) => {}// do nothing
                Err(error) => {panic!("Failed to write out file: {}", error)}
            }
        }
        None => { panic!("Failed to write out file") }
    }
}


// ------- Helpers -------

fn load_pdf(filepath: &PathBuf) -> Document {
    let doc = Document::load(filepath);
    let doc = match doc {
        Ok(d) => d,
        Err(error) => panic!("Failed to load document: \n {}", error)
    };

    doc    
}

fn delete_pages(doc: &mut Document, pages: Option<Vec<u32>>, every: Option<u32>, negate: bool) {
    match pages {
        Some(p) => {
            let page_numbers: &[u32] = &make_delete_pages_page_numbers(p, doc, negate);
            doc.delete_pages(page_numbers); // silently fails for pages outside the actual page range    
        }
        None => {
            // --every must have been used
            // TODO: see if there is a way to not check match again b/c clap will make sure that if pages is None, then every is something.
            match every {
                Some(e) => {
                    let page_numbers: &[u32] = &make_delete_every_page_numbers(e, doc, negate);
                    doc.delete_pages(page_numbers);
                }
                None => {
                    panic!("--every is not a valid integer")
                }
            }
        }
    }
}

fn make_delete_pages_page_numbers(pages: Vec<u32>, doc: &mut Document, negate: bool) -> Vec<u32> {
    if negate {
        let mut pages_set: HashSet<u32> = HashSet::new();
        // problematic only if usize is 64bits and len() is above u32::MAX
        pages_set.extend(1..=doc.get_pages().len() as u32); 
        for p in pages {
            pages_set.remove(&p);
        }
        return pages_set.into_iter().collect::<Vec<_>>();
    } else {
        return pages;
    }
}

fn make_delete_every_page_numbers(every: u32, doc: &mut Document, negate: bool) -> Vec<u32> {
    let mut pages: Vec<u32> = Vec::new();
    if negate {
        for (page_num, _) in doc.get_pages() {
            if page_num % every == 0 { pages.push(page_num); }
        }
        } else {
            for (page_num, _) in doc.get_pages() {
                if page_num % every != 0 { pages.push(page_num); }
            } 
    }
    return pages;
}