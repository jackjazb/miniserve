use crate::router::{Route, RouteMap};

// CSS is loaded at compile time
const STYLE: &str = include_str!("style.css");

pub fn render_route(route: &Route, location: &str, header_routes: &RouteMap) -> String {
    let title = format!("Miniserve");
    let body = render_body(route, location);
    render_page(&header_routes, &body, &title)
}

fn render_page(header_routes: &RouteMap, body: &str, title: &str) -> String {
    let header = render_navbar(header_routes);
    format!(
        "
	<!DOCTYPE HTML>
	<head>
		<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
		<style>
			{STYLE}
		</style>
		<title>{title}</title>
	</head>
	<body>
		{header}
		{body}
	</body>"
    )
}

fn render_navbar(header_routes: &RouteMap) -> String {
    let top_level_routes: Vec<String> = header_routes.keys().cloned().collect();
    // Filter out 'index' so it can be replaced with home. Map remaining links to HTML render fn.
    let links: String = top_level_routes
        .iter()
        .filter(|&key| key != &String::from("index"))
        .map(|route| render_link(&format!("/{route}"), route))
        .collect();

    format!("<nav>{}{links}</nav>", render_link("/", "Home"))
}

fn render_body(route: &Route, location: &str) -> String {
    match route {
        Route::Page(body) => {
            format!("{}", body)
        }
        Route::Directory(map) => {
            let sub_routes: Vec<String> = map.keys().cloned().collect();
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

/// Renders an HTML link
fn render_link(href: &str, title: &str) -> String {
    let title = strip_underscores(title);
    format!("<a href=\"{href}\">{title}</a>")
}

/// Converts strings of the form `one-two-three` to `One Two Three`
fn strip_underscores(text: &str) -> String {
    let words = text.split("_");
    words
        .fold(String::new(), |a, b| a + &b + " ")
        .trim_end()
        .into()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn render_html_link() {
        let route = "/test";
        let title = "test";
        let expected = String::from("<a href=\"/test\">test</a>");
        let result = render_link(&route, &title);
        assert_eq!(expected, result);
    }

    #[test]
    fn strip_underscores_from_string() {
        let expected = String::from("One Two Three");
        let result = strip_underscores("One_Two_Three");
        assert_eq!(expected, result);
    }

    #[test]
    fn render_single_page_body() {
        let route = &Route::Page("<h1>test</h1>".into());
        let result = render_body(route, "/");
        let expected = String::from("<h1>test</h1>");
        assert_eq!(expected, result);
    }

    #[test]
    fn render_navbar_from_routes() {
        let mut routes: RouteMap = BTreeMap::new();
        routes.insert("index".into(), Route::Page(String::new()));
        routes.insert("test-header".into(), Route::Page(String::new()));
        let result = render_navbar(&routes);
        let expected = "<nav><a href=\"/\">Home</a><a href=\"/test-header\">test-header</a></nav>";

        assert_eq!(expected, result);
    }

    #[test]
    fn render_whole_page() {
        let mut header_routes: RouteMap = BTreeMap::new();
        header_routes.insert("index".into(), Route::Page(String::new()));
        header_routes.insert("test-header".into(), Route::Page(String::new()));

        let body = "<h1>body</h1>";
        let title = "Title";

        let result = render_page(&header_routes, body, title);

        // Asserting on the entire output string is messy - testing for key elements should be enough.
        assert!(result.contains("<h1>body</h1>"));
        assert!(result.contains("Title"));
        assert!(result.contains("<a href=\"/\">Home</a>"));
        assert!(result.contains("<a href=\"/test-header\">test-header</a>"));
        assert!(result.contains(STYLE));
    }
}
