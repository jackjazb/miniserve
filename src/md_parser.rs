/**
* A basic markdown parser. Tries to parse md into html, and returns unchanged text for any failed fragments.
*/

/**
 * Parses a full markdown file into HTML.
 */
pub fn parse_md(markdown: String) -> String {
    // First split into paragraph 'blocks'
    let blocks = markdown.split("\n\n");
    let mut html = String::new();
    html += "<body style=\"font-family: helvetica\">";

    for block in blocks {
        let parsed_block = parse_block(&block);

        html += &parsed_block;
    }

    html += "</body>";
    return html;
}

/**
 * Parses blocks of markdown (lists and paragraphs etc).
 */
fn parse_block(block: &str) -> String {
    let fragments = block.split("\n");

    let mut parsed_block = String::new();
    let mut in_code_block = false;

    for fragment in fragments {
        if fragment == "```" {
            let tag = if in_code_block {
                "</div>"
            } else {
                "<div style=\"font-family:monospace\">"
            };
            in_code_block = !in_code_block;
            parsed_block.push_str(tag);

            continue;
        }

        // If we're in a code block, keep pushing lines until we get out of it
        if in_code_block {
            parsed_block.push_str(fragment);

            continue;
        }

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
    let title: String = token_split.fold(String::new(), |a, b| a + " " + b);

    // Match all valid markdown headers - if nothing matches, return the unchanged line
    let header = match first {
        Some("#") => format!("<h1>{title}</h1>"),
        Some("##") => format!("<h2>{title}</h2>"),
        Some("###") => format!("<h3>{title}</h3>"),
        _ => line.to_string(),
    };

    return header;
}
