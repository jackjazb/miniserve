# miniserve

This project aims to be a simple, very fast webserver that consumes markdown files and renders HTML.

## Usage

```text
$ miniserve ./markdown-dir
Server started in 13.7624ms // Listening on port 8080
```

`miniserve` will look for a file called index.md to use as the site root; the server will fail to start
if no such file exists.

## Options

### `-p, --port`

Overrides the default port (8080)

### `-a, --address`

Overrides the default address (127.0.0.1)

## To Do

- Config file with options
- Expand route structs with page titles etc.
- Add dates to posts
- Replace `String` key with `str` in route map
- Cache images to avoid unecessary IO
- Terminal colours
