use std::{
    collections::{HashSet, BTreeMap},
    fs,
    path::{PathBuf}
};
use lopdf::{Document, Object, ObjectId};

const VERSION: &str = "1.5";

/// Creates a silgle PDF containing all passed infiles, or all PDFs in passed directories
/// 
/// # Arguments
/// 
/// * `infiles` - a vector of PathBuf which could include directories or files
/// * `outfile` - a PathBuf representing the location to save the merged file to
/// * `compress` - a boolean flag to compress the outfile file before saving
/// 
pub fn merge(infiles: &Vec<PathBuf>, outfile: PathBuf, compress: bool) {
    // make vector of Document data structures
    let mut documents: Vec<Document> = Vec::new();
    let mut doc: Document;
    let mut document = Document::with_version(VERSION);

    let files = expand_dirs_if_necessary(infiles);

    for fname in files {
        // TODO: should not panic on I/O error as this is common. Handle better for user.
        doc = Document::load(&fname).expect("failed to open PDF");
        documents.push(doc);
    }

    merge_documents(documents, &mut document);

    if compress { document.compress(); }

    // Save the merged PDF
    save_pdf(&mut document, outfile);
}

/// Creates a single PDF containing num copies of the input PDF
/// 
/// # Arguments
/// 
/// * `infile` - a PathBuf of a single file
/// * `outfile` - a PathBuf representing the location to save the output file to
/// * `num` - a u16 integer representing the number of times to duplicate the infile
/// * `compress` - a boolean flag to compress the outfile before saving
/// 
pub fn dupe(infile: PathBuf, outfile: PathBuf, num: u16, compress: bool) {
    let doc: Document = Document::load(infile).unwrap();
    let mut documents: Vec<Document> = Vec::new();
    let mut outdoc = Document::with_version(VERSION);

    for _ in 0..num {
        documents.push(doc.clone());
    }

    merge_documents(documents, &mut outdoc);

    if compress { outdoc.compress(); }
    
    // Save the merged PDF
    save_pdf(&mut outdoc, outfile)
    // call merge but refactor merge to call a helper that operates on Document 
    // data types, rather than accepting a list of PathBuf

    // this adds a large memory overhead as we keep many copies of the same file 
    // rather than reusing a single in-memory copy of the file
    // this would have at least double the memory usage of a rused copy in the final merge

}

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

/// Rotates all pages by the input degree amount. 
/// 
/// * `infile` - a PathBuf of the file to reverse
/// * `outfile` - a PathBuf representing the location to save the output file to (Optional)
/// 
pub fn rotate(infile: PathBuf, 
              outfile: Option<PathBuf>, 
              degrees: i32, 
              pages: Option<Vec<u32>>, 
              every: Option<u32>) {
    let mut doc = load_pdf(&infile);

    match outfile {
        Some(of) => {
            rotate_doc(&mut doc.clone(), of, degrees, pages, every);
        }
        None => {
            rotate_doc(&mut doc, infile, degrees, pages, every);
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
            let page_numbers: &[u32] = &make_pages_page_numbers(p, doc, negate);
            doc.delete_pages(page_numbers); // silently fails for pages outside the actual page range    
        }
        None => {
            // --every must have been used
            // TODO: see if there is a way to not check match again b/c clap will make sure that if pages is None, then every is something.
            match every {
                Some(e) => {
                    let page_numbers: &[u32] = &make_every_page_numbers(e, doc, negate);
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
            let page_numbers = &make_pages_page_numbers(p, doc, true);
            doc.delete_pages(page_numbers);
        }
        None => {
            match every {
                Some(e) => {
                    let page_numbers = &make_every_page_numbers(e, doc, true);
                    doc.delete_pages(page_numbers);
                }
                None => {
                    panic!("--every is not a valid integer");
                }
            }
        }
    }
}

fn make_pages_page_numbers(pages: Vec<u32>, doc: &mut Document, negate: bool) -> Vec<u32> {
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

fn make_every_page_numbers(every: u32, doc: &mut Document, negate: bool) -> Vec<u32> {
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

fn rotate_doc(doc: &mut Document, filepath: PathBuf, degrees: i32, pages: Option<Vec<u32>>, every: Option<u32>) {
    match pages {
        Some(p) => {
            let page_numbers = &make_pages_page_numbers(p, doc, false);
            rotate_select_pages(doc, page_numbers, degrees);
        }
        None => {
            match every {
                Some(e) => {
                    let page_numbers = &make_every_page_numbers(e, doc, false);
                    rotate_select_pages(doc, page_numbers, degrees);
                }
                None => {
                    rotate_all_pages(doc, degrees);
                }
            }
        }
    }

    save_pdf(doc, filepath);
}

fn rotate_select_pages(doc: &mut Document, page_numbers: &Vec<u32>, degrees: i32) {
    let pages: BTreeMap<u32, ObjectId> = doc.get_pages();

    for p in page_numbers {
        let object_id = pages.get(p);
        match object_id {
            Some(id) => {
                if let Ok(page) = doc.get_object(*id) {
                    if let Ok(dict) = page.as_dict() {
                        let mut dict = dict.clone();
                        
                        dict.set("Rotate", degrees);
                        doc.objects.insert(*id, Object::Dictionary(dict));
                    }
                }
            }
            None => {} // do nothing, but TODO: consider accumulating missed pages to output to user
        }
    }
}

fn rotate_all_pages(doc: &mut Document, degrees: i32) {
    for (_, object_id) in doc.get_pages() {
        if let Ok(page) = doc.get_object(object_id) {
            if let Ok(dict) = page.as_dict() {
                let mut dict = dict.clone();
                
                dict.set("Rotate", degrees);
                doc.objects.insert(object_id, Object::Dictionary(dict));
            }
        }
    }
}

// check if any of the entries are directories, if they are, expand the vector to include
// all PDFs in the directory (do not search subdirs)
fn expand_dirs_if_necessary(infiles: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut dir_pdf_files: Vec<PathBuf>;
    let mut expanded = Vec::with_capacity(infiles.len());

    for path in infiles {
        if path.is_dir() {
            dir_pdf_files = get_files_from_dir(path).expect("failed to get files from directory");
            expanded.append(&mut dir_pdf_files);
        } else  {
            expanded.push(path.to_path_buf());
        }
    }

    return expanded;
}

// https://stackoverflow.com/questions/58062887/filtering-files-or-directories-discovered-with-fsread-dir
fn get_files_from_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter(|result| result.is_ok()) // filter to non Err results
        .map(|result| result.unwrap().path()) // turn valid DirEntries into PathBufs
        .filter(|result| 
            result.is_file() && 
            result.extension() != None && 
            (result.extension().unwrap() == "pdf" || result.extension().unwrap() == "PDF")) // filter to only files
        .collect()
    )
}

// this is almost unmodified from the examples in the lopdf README https://github.com/J-F-Liu/lopdf
// TODO: consider refactoring
// FIXME: this is broken for files with multiple Pages objects (I think)
fn merge_documents(documents: Vec<Document>, outdoc: &mut Document) {
    // Define a starting max_id (will be used as start index for object_ids)
    let mut max_id = 1;
    // let mut pagenum = 1;
    // Collect all Documents Objects grouped by a map
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();

    for mut doc in documents {
        // let mut first = false;

        // renumber the current doc starting with the current max_id
        doc.renumber_objects_with(max_id);
        // sets the new max_id to the id of the last page of the current doc + 1 so that the next doc starts in the correct location
        max_id = doc.max_id + 1; 

        // extend the documents_pages with a BTreeMap of ObjectId and Object which is a enum of Object types
        // An object can be:
        /*
            pub enum Object {
                Null,
                Boolean(bool),
                Integer(i64),
                Real(f64),
                Name(Vec<u8>),
                String(Vec<u8>, StringFormat),
                Array(Vec<Object>),
                Dictionary(Dictionary),
                Stream(Stream),
                Reference(ObjectId),
            }
        */
        documents_pages.extend(
            doc
                    .get_pages()
                    .into_iter()
                    .map(|(_, object_id)| (object_id, doc.get_object(object_id).unwrap().to_owned(),))
                    .collect::<BTreeMap<ObjectId, Object>>(),
        );

        // add all the objects from each document to a collection
        documents_objects.extend(doc.objects);
    }

    // Catalog and Pages are mandatory 
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all objects except "Page" type
    for (object_id, object) in documents_objects.iter() {
        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects
        // All other objects should be collected and inserted into the main Document
        match object.type_name().unwrap_or("") {
            "Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages"
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        *object_id
                    },
                    object.clone(),
                ));
            }
            "Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            *object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            "Page" => {}     // Ignored, processed later and separately
            "Outlines" => {} // Ignored, not supported yet
            "Outline" => {}  // Ignored, not supported yet
            _ => {
                outdoc.objects.insert(*object_id, object.clone());
            }
        }
    }

    // If no "Pages" found abort
    if pages_object.is_none() {
        println!("Pages root not found.");

        return;
    }

    // Iter over all "Page" and collect with the parent "Pages" created before
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);

            outdoc
                    .objects
                    .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    // If no "Catalog" found abort
    if catalog_object.is_none() {
        println!("Catalog root not found.");

        return;
    }

    let catalog_object = catalog_object.unwrap();
    let pages_object = pages_object.unwrap();

    // Build a new "Pages" with updated fields
    if let Ok(dictionary) = pages_object.1.as_dict() {
        let mut dictionary = dictionary.clone();

        // Set new pages count
        dictionary.set("Count", documents_pages.len() as u32);

        // Set new "Kids" list (collected from documents pages) for "Pages"
        dictionary.set(
            "Kids",
            documents_pages
                    .into_iter()
                    .map(|(object_id, _)| Object::Reference(object_id))
                    .collect::<Vec<_>>(),
        );

        outdoc
                .objects
                .insert(pages_object.0, Object::Dictionary(dictionary));
    }

    // Build a new "Catalog" with updated fields
    if let Ok(dictionary) = catalog_object.1.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", pages_object.0);
        dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

        outdoc
                .objects
                .insert(catalog_object.0, Object::Dictionary(dictionary));
    }

    outdoc.trailer.set("Root", catalog_object.0);

    // Update the max internal ID as wasn't updated before due to direct objects insertion
    outdoc.max_id = outdoc.objects.len() as u32;

    // Reorder all new Document objects
    outdoc.renumber_objects();

     //Set any Bookmarks to the First child if they are not set to a page
    outdoc.adjust_zero_pages();

    //Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
    if let Some(n) = outdoc.build_outline() {
        if let Ok(x) = outdoc.get_object_mut(catalog_object.0) {
            if let Object::Dictionary(ref mut dict) = x {
                dict.set("Outlines", Object::Reference(n));
            }
        }
    }
}
