# Made-up
[![Travis](https://img.shields.io/travis/maccoda/made-up.svg)]()
![https://ci.appveyor.com/api/projects/status/?svg=true](https://ci.appveyor.com/api/projects/status/github/maccoda/made-up?svg=true)
[![Codecov](https://img.shields.io/codecov/c/github/maccoda/made-up.svg)]()
[![GitHub release](https://img.shields.io/github/release/maccoda/made-up.svg)]()

Simple tool to generate a static website from a directory structure of Markdown
files. Bringing back the old school simplicity and beautifying notes and
documents.

## Usage
### Configuration
Made-Up will look for a configuration file `mdup.yml` in the root directory
which allows the user to have a bit more control over the end product. This
configuration file is not mandatory thus all fields are optional. If not
specified the default will be used.

| Field | Type | Description | Default |
|---|---| --- | --- |
|`stylesheet` | String | Relative path to the style sheet to use for the site|  |
|`index` | Boolean | Choose if index page is to be generated. | *False* |
| `out_dir`| String | Path to the output directory. | *./out*|

## Ignored Files
Not every file can make the cut for the site, so to make it easy to determine
which ones do we have a simple convention. All files and directories that are to
be ignored should begin with an underscore, all other markdown files will be a
part of the site.

## TODO

### Functional
- [X] Perform the basic conversion of markdown to HTML
- [ ] Traverse directories to:
  - [ ] Create page hierarchy
  - [X] Discover all markdown to be used
- [ ] Auto-generate the index page as just a listing of all top level pages
(Partially complete, it generates for all files not just top level)
- [ ] Shall provide a configuration where:
  - [ ] Can excluded certain files from being part of the site
  - [ ] Can specify user defined index instead of generated
  - [X] Allow for a stylesheet to be specified
- [X] Internal linking to sections within same page
- [X] Linking to other pages (Need to specify how to do this, end in *.html)
- [ ] Copy images

### Technical
- [X] Get a logger in here please, no more println
- [ ] Ensure properly handle errors
- [ ] More testing!!!
