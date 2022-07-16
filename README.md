# PDFH - the PDF hammer
PDFH is a command-line tool for transforming PDF files.

```
USAGE:
    pdfh <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    delete     Delete pages from a PDF. A list of space separated pages or --every ith page
    dupe       Duplicates a PDF n times and saves the duplicates into a single file
    extract    Extract specitic pages from a PDF
    help       Print this message or the help of the given subcommand(s)
    merge      Merges PDFs into a single file
    reverse    Reverse the order of a PDF
    rotate     Rotate an entire document, or select pages
    split      Splits each page of a PDF into a separate file
```

## The Name
While writing and picking a name for this tool, I discovered both [QPDF](https://github.com/qpdf/qpdf) and [PDFtk ("tool kit") Server](https://www.pdflabs.com/tools/pdftk-server/), both of which offer many more features. This project is definitely not a tool kit, but maybe it's a single tool: a hammer. It's not always the perfect tool for the job, but if you only need something simple to then a hammer might do.

## Installation Instructions
