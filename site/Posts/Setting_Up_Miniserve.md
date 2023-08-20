# Setting Up Miniserve

This is a quick guide on using `miniserve` to build your own static website. This page is a work in
progress and will likely change lots as features are added!

## Setup

- Create a folder called `site` in the same directory as the `miniserve` binary
- Create an `index.md` file in the root of this directory - this will be served as the default route
- Populate the folder with markdown files. Case in filenames is preserved, and underscores(`_`) are converted to spaces when rendered. You can organise your pages into folders, currently to a depth of one. The layout of the site mirrors that of the directory, with the top level items in the `site` directory comprising the navigation bar links
- Run `miniserve`
