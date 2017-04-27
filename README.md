# Made-up
[![Travis](https://img.shields.io/travis/maccoda/made-up.svg)]()
[![AppVeyor](https://img.shields.io/appveyor/ci/maccoda/made-up.svg)]()

Simple tool to generate a static website from a directory structure of Markdown
files. Bringing back the old school simplicity and beautifying notes and
documents.

## TODO

### Functional
- [X] Perform the basic conversion of markdown to HTML
- [ ] Traverse directories to:
  - [ ] Create page heirarchy
  - [X] Discover all markdown to be used
- [-] Auto-generate the index page as just a listing of all top level pages
(Partially complete, it generates for all files not just top level)
- [ ] Shall provide a configuration where:
  - [ ] Can excluded certain files from being part of the site
  - [ ] Can specify user defined index instead of generated
  - [ ] Allow for a stylesheet to be specified
- [X] Internal linking to sections within same page
- [ ] Linking to other pages (Need to specify how to do this, end in *.html)

### Technical
- [ ] Get a logger in here please, no more println
- [ ] Ensure properly handle errors
- [ ] More testing!!!
