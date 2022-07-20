use std::{collections::{HashSet}, path::{PathBuf}};
use lopdf::{Document, Object};

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
    
    match outfile {
        Some(f) => {
            save_pdf(&mut doc, f);
        }
        None => {
            save_pdf(&mut doc, infile);
        }
    }
}

/// Extracts the pages listed in --pages, or every --every page in a PDF
/// 
/// * `infile` - a PathBuf of a single file
/// * `outfile` - a PathBuf representing the location to save the output file to
/// * `pages` - a list of page numbers to delete
/// * `every` - an integer 
/// * `negate` - negates/inverts the --page or --every selection, instead keeping only those pages listed
/// 
pub fn extract(infile: PathBuf, outfile: PathBuf, pages: Option<Vec<u32>>, every: Option<u32>) {
    let mut doc = Document::load(&infile).expect("failed to open PDF");

    extract_pages(&mut doc, pages, every);

    save_pdf(&mut doc, outfile);
}



/// Reverses the page order of a document either inplace or in a new file
/// 
/// * `infile` - a PathBuf of the file to reverse
/// * `outfile` - a PathBuf representing the location to save the output file to (Optional)
/// 
pub fn reverse(infile: PathBuf, outfile: Option<PathBuf>) {
    let mut doc = load_pdf(&infile);

    match outfile {
        Some(of) => {
            reverse_doc(&mut doc.clone(), of);
        }
        None => {
            reverse_doc(&mut doc, infile);
        }
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

fn save_pdf(doc: &mut Document, filepath: PathBuf) {
    doc.prune_objects();
    doc.adjust_zero_pages();
    doc.build_outline();
    doc.delete_zero_length_streams();

    if doc.get_pages().len() == 0 { panic!("Resulting document would have no pages."); }

    let result = doc.save(filepath);
    match result {
        Ok(_) => {}// do nothing
        Err(error) => {panic!("Failed to write out file: {}", error)}
    }
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

fn extract_pages(doc: &mut Document, pages: Option<Vec<u32>>, every: Option<u32>) {
    match pages {
        Some(p) => {
            let page_numbers = &make_delete_pages_page_numbers(p, doc, true);
            doc.delete_pages(page_numbers);
        }
        None => {
            match every {
                Some(e) => {
                    let page_numbers = &make_delete_every_page_numbers(e, doc, true);
                    doc.delete_pages(page_numbers);
                }
                None => {
                    panic!("--every is not a valid integer");
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
        for p in pages.iter() {
            pages_set.remove(p);
        }
        pages_set.into_iter().collect::<Vec<_>>()
    } else {
        pages
    }
}

fn make_delete_every_page_numbers(every: u32, doc: &mut Document, negate: bool) -> Vec<u32> {
    let mut pages: Vec<u32> = Vec::new();
    if negate {
        for (page_num, _) in doc.get_pages() {
            if page_num % every != 0 { pages.push(page_num); }
        }
        } else {
            for (page_num, _) in doc.get_pages() {
                if page_num % every == 0 { pages.push(page_num); }
            } 
    }
    pages
}

fn reverse_doc(doc: &mut Document, filepath: PathBuf) {
    // Try getting the Kids reference table from Pages and reversing the vector of references
    // do this for all Pages objects as there may be more than one in the 

    // inefficient to scan every object in the document when we are only looking for Pages
    for (object_id, object) in doc.clone().objects.iter() { // TODO: fix wasteful clone
        match object.type_name().unwrap_or("") {
            "Pages" => {
                if let Ok(dict) = object.as_dict() {
                    let mut dict = dict.clone();
                    let kids_refs = dict.get(b"Kids");
                    match kids_refs {
                        Ok(ref_arr) => {
                            // get the Pages object, pull the Kids and reverse the array of page references
                            // then replace the entire Pages object using the original object_id
                            let mut arr = ref_arr.as_array().unwrap().clone();
                            println!("{:?}", arr);
                            arr.reverse();
                            println!("{:?}", arr);
                            dict.set("Kids", Object::Array(arr));

                            doc.objects.insert(*object_id, Object::Dictionary(dict));
                        }
                        Err(error) => { println!("{}", error); } // TODO: temp, will leak impl details
                    }
                }
            }
            _ => {} // do nothing for all other object types
        }
    }

    save_pdf(doc, filepath);
}
