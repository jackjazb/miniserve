use crate::router::{Route, RouteMap};

const GLOBAL_STYLE: &str = include_str!("style.css");

pub fn render_route(route: &Route, location: &str, header_routes: &RouteMap) -> String {
    let body = render_body(route, location);
    render_page(&header_routes, &body)
}

fn render_page(header_routes: &RouteMap, body: &str) -> String {
    let top_level_routes: Vec<String> = header_routes.keys().cloned().collect();
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
		<style>
			{GLOBAL_STYLE}
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
            let sub_routes: Vec<String> = map.keys().cloned().collect();
            sub_routes
                .iter()
                .map(|route| {
                    let href = format!("{location}/{route}");
                    render_link(&href, &route)
                })
                .collect()
        }
    }
}

/// Renders an HTML link, stripping leading slashes from the visible title.
fn render_link(href: &str, title: &str) -> String {
    if title.starts_with("/") {
        title.to_string().remove(0);
    }
    format!("<a href=\"{href}\">{title}</a></div>")
}
