use crate::router::{Route, RouteMap};

const STYLE: &str = include_str!("style.css");
const TITLE: &str = "Miniserve";

pub fn render_route(route: &Route, location: &str, header_routes: &RouteMap) -> String {
    let body = render_body(route, location);
    render_page(&header_routes, &body)
}

fn render_page(header_routes: &RouteMap, body: &str) -> String {
    let header = render_navbar(header_routes);
    format!(
        "
	<!DOCTYPE HTML>
	<head>
		<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
		<style>
			{STYLE}
		</style>
		<title>{TITLE}</title>
	</head>
	<body>
		{header}
		{body}
	</body>"
    )
}

fn render_navbar(header_routes: &RouteMap) -> String {
    let top_level_routes: Vec<String> = header_routes.keys().cloned().collect();
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

/// Renders an HTML link, stripping leading slashes from the visible title.
fn render_link(href: &str, title: &str) -> String {
    if title.starts_with("/") {
        title.to_string().remove(0);
    }
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
    use super::*;

    #[test]
    fn render_html_link_with_leading_slash_removed() {
        let route = "/test";
        let expected = String::from("<a href=\"/test\">test</a>");
        let result = render_link(&route, &route);
        assert_eq!(expected, result);
    }

    #[test]
    fn strip_underscores_from_string() {
        let expected = String::from("One Two Three");
        let result = strip_underscores("One_Two_Three");
        assert_eq!(expected, result);
    }
}
