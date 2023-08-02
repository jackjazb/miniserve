use std::collections::HashMap;

// Maps strings (e.g. 'posts') to routes
pub type RouteMap = HashMap<String, Route>;

/// Represents a resource on the webserver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    /// A single page, represented as an HTML string.
    Page(String),
    /// A directory, represented by a sub route map.
    Directory(RouteMap),
}

/// Takes a slash delimited route and a RouteMap, and parses the route.
/// Returns the resource pointed to by the route if it exists, None if not.
pub fn parse_route(route: &str, base_routes: &RouteMap) -> Option<Route> {
    // Any special redirects (eg / => /index) are defined here
    let route = match route {
        "/" => "/index",
        _ => route,
    };
    let mut split = route.split("/");

    // Check each map and sub map for each segment of the route
    let mut current_route_map = base_routes;
    while let Some(part) = split.next() {
        if part == "" {
            continue;
        }
        match current_route_map.get(part) {
            Some(route) => match route {
                Route::Page(_) => return Some(route.clone()),
                Route::Directory(sub_routes) => current_route_map = sub_routes,
            },
            // If the key does not exist in the route map, return None
            None => return None,
        }
    }
    // If we've run out of route components, render the last route map as a directory page
    Some(Route::Directory(current_route_map.clone()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn create_route_map() -> RouteMap {
        let sub_route_map = HashMap::from([(
            "sub-page".to_string(),
            Route::Page("sub-page-html".to_string()),
        )]);

        let route_map = HashMap::from([
            ("index".to_string(), Route::Page("index-html".to_string())),
            ("sub-route".to_string(), Route::Directory(sub_route_map)),
        ]);

        route_map
    }

    #[test]
    fn load_top_level_route() {
        let route_map = create_route_map();
        let page = parse_route("/index", &route_map);
        assert_eq!(page, Some(Route::Page("index-html".into())));
    }

    #[test]
    fn load_index() {
        let route_map = create_route_map();
        let page = parse_route("/", &route_map);
        assert_eq!(page, Some(Route::Page("index-html".into())));
    }

    #[test]
    fn load_sub_route() {
        let route_map = create_route_map();
        let page = parse_route("/sub-route/sub-page", &route_map);
        assert_eq!(page, Some(Route::Page("sub-page-html".into())));
    }

    #[test]
    fn load_dead_route() {
        let route_map = create_route_map();
        let page = parse_route("null", &route_map);
        assert_eq!(page, None);
    }
}
