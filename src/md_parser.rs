use std::str::Split;

/**
* A basic markdown parser. Tries to parse md into html, and returns unchanged text for any failed fragments.
*/
#[derive(PartialEq)]
enum Block {
    Root,
    Code,
    List,
}

/**
 * Parses a full markdown file into HTML.
 */
pub fn parse_md(markdown: String) -> String {
    // First split into paragraph 'blocks'
    let blocks = markdown.split("\n\n");
    let mut html = String::new();

    for block in blocks {
        let parsed_block = parse_block(&block);
        html += &parsed_block;
    }

    return html;
}

/**
 * Parses blocks of markdown (lists and paragraphs etc).
 */
fn parse_block(block: &str) -> String {
    let fragments = block.split("\n");

    let mut parsed_block = String::new();

    // Indicates which type of block is currently being rendered
    let mut current_block: Block = Block::Root;

    for fragment in fragments {
        // Don't try to parse an empty line
        if fragment == "" {
            continue;
        }
        // Upon encountering a "-", enter a list block. Stay in a list block until a line without a dash is encountered
        if &fragment[..1] == "-" {
            if current_block != Block::List {
                parsed_block.push_str("<ul>");
                current_block = Block::List;
            }
            let mut tokens = fragment.split(" ");
            tokens.next();
            let text = get_tail(tokens);
            parsed_block.push_str(&format!("<li>{text}</li>"));
            continue;
        }

        if current_block == Block::List {
            parsed_block.push_str("</ul>");
            current_block = Block::Root;
        }

        // Handle getting in and out of code blocks
        if fragment == "```" {
            if current_block == Block::Code {
                current_block = Block::Root;
                parsed_block.push_str("</div>");
            } else {
                current_block = Block::Code;
                parsed_block.push_str("<div class=\"code\">");
            };

            continue;
        }

        // If we're in a code block, keep pushing lines until we get out of it
        if current_block == Block::Code {
            parsed_block.push_str(fragment);
            parsed_block.push_str("</br>");
            continue;
        }

        // If we're not in a block, parse the line as a whole
        parsed_block.push_str(&parse_line(fragment));
    }

    parsed_block
}

/**
 * Parses fragments (i.e lines) of markdown
 *
 */
fn parse_line(line: &str) -> String {
    let index = 0;
    let chars = line.chars().peekable();
    for char in chars {
        // If the first character is #, render a header
        if char == '#' && index == 0 {
            return parse_header(line);
        }
    }
    let mut parsed_line = String::from("<p>");
    parsed_line.push_str(line);
    parsed_line.push_str("</p>");
    parsed_line
}

/**
 * Takes a line and attempts to parse a header from it
 */
fn parse_header(line: &str) -> String {
    let mut token_split = line.split(" ");

    let first = token_split.next();
    // If splitting on space failed, return just the line
    if first.is_none() {
        return line.to_string();
    }

    // Fold remaining tokens back into a string
    let title: String = get_tail(token_split);

    // Match all valid markdown headers - if nothing matches, return the unchanged line
    let header = match first {
        Some("#") => format!("<h1>{title}</h1>"),
        Some("##") => format!("<h2>{title}</h2>"),
        Some("###") => format!("<h3>{title}</h3>"),
        _ => line.to_string(),
    };

    return header;
}

/**
 * Utility function for getting text after a bit of markdown syntax
 */
fn get_tail(tokens: Split<&str>) -> String {
    tokens.fold(String::new(), |a, b| a + " " + b)
}
