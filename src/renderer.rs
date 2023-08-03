use crate::router::{Route, RouteMap};

const STYLE: &str = include_str!("style.css");

pub fn render_route(route: &Route, location: &str, header_routes: &RouteMap) -> String {
    let body = render_body(route, location);
    render_page(&header_routes, &body)
}

fn render_page(header_routes: &RouteMap, body: &str) -> String {
    let mut top_level_routes: Vec<String> = header_routes.keys().cloned().collect();
    top_level_routes.sort();
    let links: String = top_level_routes
        .iter()
        .filter(|&key| key != &String::from("index"))
        .map(|route| render_link(&format!("/{route}"), route))
        .collect();

    let header = format!("<nav>{}{links}</nav>", render_link("/", "home"));

    format!(
        "
	<!DOCTYPE HTML>
	<head>
		<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
		<style>
			{STYLE}
		</style>
	</head>
	<body>
		{header}
		{body}
	</body>"
    )
}

fn render_body(route: &Route, location: &str) -> String {
    match route {
        Route::Page(body) => {
            format!("{}", body)
        }
        Route::Directory(map) => {
            let mut sub_routes: Vec<String> = map.keys().cloned().collect();
            sub_routes.sort();
            let links: String = sub_routes
                .iter()
                .map(|route| {
                    let href = format!("{location}/{route}");
                    format!("<li>{}</li>", render_link(&href, &route))
                })
                .collect();
            format!("<ul>{links}</ul>")
        }
    }
}

/// Renders an HTML link, stripping leading slashes from the visible title.
fn render_link(href: &str, title: &str) -> String {
    if title.starts_with("/") {
        title.to_string().remove(0);
    }
    let title = dash_to_sentence_case(title);
    format!("<a href=\"{href}\">{title}</a></div>")
}

/// Converts strings of the form `one-two-three` to `One Two Three`
fn dash_to_sentence_case(text: &str) -> String {
    let words = text.split("-");
    words
        .map(|w| uppercase(w))
        .fold(String::new(), |a, b| a + &b + " ")
        .trim_end()
        .into()
}

/// Converts the first letter of a string to be uppercase
fn uppercase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}
