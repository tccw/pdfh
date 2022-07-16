#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use pdfh::utils;

    const DATA_DIR: &str = "test-data";

    fn build_filepath(filename: &str) -> String {
        return format!("{}/{}", DATA_DIR, filename)
    }

    // https://www.reddit.com/r/rust/comments/9hqi44/what_is_the_status_of_improved_testing_in_rust/
    struct TestResources {
        two_pages: PathBuf,
        multi_page: PathBuf,
        file_bad_header: PathBuf,
        file_does_not_exist: PathBuf,
        outfile_valid: PathBuf,
        outfile_cannot_write: PathBuf
    }
    
    impl TestResources {
        fn new() -> TestResources {
            TestResources {
                two_pages: PathBuf::from(build_filepath("two-pages.pdf")),
                multi_page: PathBuf::from(build_filepath("unique-multi-page.pdf")),
                file_bad_header: PathBuf::from(build_filepath("notapdf.txt")),
                file_does_not_exist: PathBuf::from(build_filepath("does-not-exist.pdf")),
                outfile_valid: PathBuf::from(build_filepath("output/outfile.pdf")),
                outfile_cannot_write: PathBuf::from(build_filepath("nonexistentdir/outfile.pdf"))
            }
        }
    }

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
    fn write_out_success() {
        let test_resource: TestResources = TestResources::new();

        let every = None;
        let pages = Some(vec![1,3]);
        let outfile = Some(test_resource.outfile_valid);

        utils::delete(test_resource.multi_page, outfile, pages, every, false, false)
    }


}