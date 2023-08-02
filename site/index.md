# miniserve

This project aims to be a simple, very fast webserver that consumes markdown files and renders HTML.

## Usage

- Place markdown files in a directory called `site` in the same directory as the server binary
- Run the server

The program loads all the markdown from the site directory on startup, and exposes an HTTP server. The site structure mirrors the directory structure on disk - i.e. `./site/posts/first-post.md` would be accessible at `localhost/posts/first-post`.

Directories are rendered as lists of links.

## To Do

- Add a site directory command line argument
- Navbar
- Styling in a separate file
- `favicon.ico`

 ![This site's icon](/favicon.ico)
