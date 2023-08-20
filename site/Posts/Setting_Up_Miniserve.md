# Setting Up Miniserve

This is a quick guide on using `miniserve` to build your own static website.

## Setup

- Create a new directory containing an `index.md` file at the root - this will be served as the default route.
- Put further markdown files in this directory. Directories can also be added - these will be added to the navbar.
- Filenames should be formatted in sentence case, separated by underscores (e.g. `First_Post.md`).
- Run miniserve as follows:

```text
$ miniserve ./you-directory-name
Server started in 13.7624ms // Listening on port 8080
```
