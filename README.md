# Made-up
 [![Travis Linux](https://img.shields.io/travis/maccoda/made-up.svg)](https://travis-ci.org/maccoda/made-up)
 ![Appveyor Windows](https://ci.appveyor.com/api/projects/status/github/maccoda/made-up?svg=true)
[![Codecov](https://img.shields.io/codecov/c/github/maccoda/made-up.svg)](https://codecov.io/gh/maccoda/made-up)
[![GitHub release](https://img.shields.io/github/release/maccoda/made-up.svg)](https://github.com/maccoda/made-up/releases)

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
|`stylesheet` | Array<String> | List of paths for the stylesheets used for the site. | *[]* |
|`index_template` | String | Path to the user defined template to use to generate the index page. This must be a relative path from the root directory. | *''* |
| `out_dir`| String | Path to the output directory. | *./out*|
| `copy_resources` | Boolean | Specify if the stylesheet and images are to be copied or just referenced. | *True* |
| `title` | String | Title to be in the `<head>` of the generated site. | *Title* |

### Ignored Files
Not every file can make the cut for the site, so to make it easy to determine
which ones do we have a simple convention. All files and directories that are to
be ignored should begin with an underscore, all other markdown files will be a
part of the site.

### Index Template
To allow for user defined index page but not needing to maintain the current
list of pages, the user is able to define the path to a Handlebars template to be used to generate the index page. If this option is not provided the default template defined by [index.hbs](templates/index.hbs) will be applied.

The elements of the site are provided under the list `element` which has two
properties: `path` and `heading`, representing the path to the generated HTML
and the top level heading respectively.

### Images
To make things simple there is a simple convention for where to store your
images so Made-Up knows where to get them from when generating your site. Who
would have guessed it but if you put all your images in a directory in your root
directory named **images** Made-Up will ensure these come across to the site.
They will be copied across into another **images** directory so you will already
doing the hard job of correctly referencing for Made-Up :).

