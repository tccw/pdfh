#[cfg(test)]

mod tests {
    use std::path::PathBuf;

    use pdfh::utils;
    use ::function_name::named;


    const DATA_DIR: &str = "test-data";

    fn build_filepath(filename: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}", DATA_DIR, filename))
    }

    fn build_outfile_pathbuf(filename: &str) -> PathBuf {
        PathBuf::from(format!("{}/output/{}.pdf", DATA_DIR, filename))
    }

    // https://www.reddit.com/r/rust/comments/9hqi44/what_is_the_status_of_improved_testing_in_rust/
    struct TestResources {
        single_page: PathBuf,
        two_pages: PathBuf,
        multi_page_single_page_obj: PathBuf,
        multi_page_multiple_pages_obj: PathBuf,
        file_bad_header: PathBuf,
        file_does_not_exist: PathBuf,
        outfile_valid: PathBuf,
        outfile_cannot_write: PathBuf
    }
    
    impl TestResources {
        fn new() -> TestResources {
            TestResources {
                single_page: build_filepath("one-page-with-image.pdf"),
                two_pages: build_filepath("two-pages.pdf"),
                multi_page_single_page_obj: build_filepath("single-pages-object-multi-page.pdf"),
                multi_page_multiple_pages_obj: build_filepath("multiple-pages-objects-multi-page.pdf"),
                file_bad_header: build_filepath("notapdf.txt"),
                file_does_not_exist: build_filepath("does-not-exist.pdf"),
                outfile_valid: build_filepath("output/outfile.pdf"),
                outfile_cannot_write: build_filepath("nonexistentdir/outfile.pdf")
            }
        }
    }

    // fn compare_two_pdfs(doc0: lopdf::Document, doc1: lopdf::Document) -> bool {
    //     return false;
    // }

    #[test]
    #[should_panic(expected = "Failed to load document: \n Invalid file header")]
    fn invalid_pdf_file() {
        let test_resource: TestResources = TestResources::new();

        let every = None;
        let pages = Some(vec![1,3]);
        let outfile = None;

        utils::delete(test_resource.file_bad_header, outfile, pages, every, false, false)
    }

    #[test]
    #[should_panic(
        expected = "Failed to load document: \n No such file or directory (os error 2)"
    )]
    fn infile_not_found() {
        let test_resource: TestResources = TestResources::new();

        let every = None;
        let pages = Some(vec![1,3]);
        let outfile = None;

        utils::delete(test_resource.file_does_not_exist, outfile, pages, every, false, false)
    }

    #[test]
    #[should_panic(expected = "Failed to write out file: No such file or directory (os error 2)")]
    fn cannot_write_outfile() {
        let test_resource: TestResources = TestResources::new();

        let every = None;
        let pages = Some(vec![1,3]);
        let outfile = Some(test_resource.outfile_cannot_write);

        utils::delete(test_resource.two_pages, outfile, pages, every, false, false)
    }

    #[test]
    fn delete_write_out_success() {
        let test_resource: TestResources = TestResources::new();

        let every = None;
        let pages = Some(vec![1,3]);
        let outfile = Some(test_resource.outfile_valid);

        utils::delete(test_resource.multi_page_single_page_obj, outfile, pages, every, false, false)
    }

    #[test]
    #[named]
    fn reverse_write_out_success() {
        let test_resource: TestResources = TestResources::new();

        let outfile = Some(build_outfile_pathbuf(function_name!()));
        utils::reverse(test_resource.two_pages, outfile);
    }

    // Visual inspection is required of the output of these tests
    #[test]
    #[named]
    fn reverse_doc_with_intermediate_pages_objects() {
        let test_resource: TestResources = TestResources::new();

        let outfile = Some(build_outfile_pathbuf(function_name!()));
        utils::reverse(test_resource.multi_page_multiple_pages_obj, outfile);
    }

    #[test]
    #[named]
    fn reverse_doc_with_single_pages_object() {
        let test_resource: TestResources = TestResources::new();

        let outfile = Some(build_outfile_pathbuf(function_name!()));
        utils::reverse(test_resource.multi_page_single_page_obj, outfile);
    }

    #[test]
    #[named]
    fn reverse_single_page_document() {
        let test_resource: TestResources = TestResources::new();

        let outfile = Some(build_outfile_pathbuf(function_name!()));
        utils::reverse(test_resource.single_page, outfile);
    }
}