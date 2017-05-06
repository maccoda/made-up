# Made-up
|||||
|---|---|---|---|
| Linux | [![Travis](https://img.shields.io/travis/maccoda/made-up.svg)]() | Windows |![https://ci.appveyor.com/api/projects/status/?svg=true](https://ci.appveyor.com/api/projects/status/github/maccoda/made-up?svg=true) |
|[![Codecov](https://img.shields.io/codecov/c/github/maccoda/made-up.svg)]()|[![GitHub release](https://img.shields.io/github/release/maccoda/made-up.svg)]()|||

Simple tool to generate a static website from a directory structure of Markdown
files. Bringing back the old school simplicity and beautifying notes and
documents.

## Usage
Very very simple. Run the executable/binary `made-up` in one of the two ways:
1. When an argument is provided it expects this to be the root directory for all your Markdown files.
   ```
   > made-up /root/dir/path
   ```
1. When no argument is provided it assumes it is in the root directory and will start from there.
   ```
   > made-up
   ```
Pretty simple huh? Just make sure you have the correct set up as explained below
and you are good to go!

## Set up
### Configuration
Made-Up will look for a configuration file `mdup.yml` in the root directory
which allows the user to have a bit more control over the end product. This
configuration file is not mandatory thus all fields are optional. If not
specified the default will be used.

| Field | Type | Description | Default |
|---|---| --- | --- |
|`stylesheet` | String | Relative path to the style sheet to use for the site|  |
|`gen_index` | Boolean | Choose if index page is to be generated. | *False* |
| `out_dir`| String | Path to the output directory. | *./out*|

### Ignored Files
Not every file can make the cut for the site, so to make it easy to determine
which ones do we have a simple convention. All files and directories that are to
be ignored should begin with an underscore, all other markdown files will be a
part of the site.

### Index Page
If the configuration property of `index` is set to *False*, that is the user has
specified their own index file, Made-Up will expect to find a `index.md` in
the root directory otherwise it will fail.

### Images
To make things simple there is a simple convention for where to store your images so Made-Up knows where to get them from when generating your site. Who would have guessed it but if you put all your images in a directory in your root directory named **images** Made-Up will ensure these come across to the site. They will be copied across into another **images** directory so you will already doing the hard job of correctly referencing for Made-Up :).

## TODO

### Functional
- [X] ~~Perform the basic conversion of markdown to HTML~~
- [ ] Traverse directories to:
  - [ ] Create page hierarchy
  - [X] ~~Discover all markdown to be used~~
- [ ] Auto-generate the index page as just a listing of all top level pages
(Partially complete, it generates for all files not just top level)
- [X] ~~Shall provide a configuration where:~~
  - [X] ~~Can excluded certain files from being part of the site~~
  - [X] ~~Can specify user defined index instead of generated~~
  - [X] ~~Allow for a stylesheet to be specified~~
- [X] ~~Internal linking to sections within same page~~
- [X] ~~Linking to other pages (Need to specify how to do this, end in *.html)~~
- [X] ~~Copy images~~

### Technical
- [X] ~~Get a logger in here please, no more println~~
- [ ] Ensure properly handle errors
- [ ] More testing!!! Let's try get 95% coverage!
- [ ] Tidy up code structure (particularly lib.rs)
