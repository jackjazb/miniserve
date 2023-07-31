use std::collections::HashMap;

// Maps strings (e.g. 'posts') to routes
pub type RouteMap = HashMap<String, Route>;

/// Represents a route.
/// This can either be a page or a set of sub-routes
#[derive(Debug)]
pub enum Route {
    Page(String),
    SubRouteMap(RouteMap),
}

/// Takes a slash delimited string and a RouteMap, and returns the route HTML defined in the map.
/// Returns 'None' if the route does not exist.
pub fn load_route(route: &str, route_map: &RouteMap) -> Option<String> {
    // Any special redirects (eg / => /index) are defined here
    let route = match route {
        "/" => "/index",
        _ => route,
    };

    let mut split = route.split("/");

    let mut current_route_map = route_map;

    // Iterate over each part of the route, identifying
    while let Some(part) = split.next() {
        if let Some(route) = current_route_map.get(part) {
            match route {
                Route::Page(html) => return Some(html.to_string()),
                Route::SubRouteMap(route_map) => current_route_map = route_map,
            }
        }
    }
    // If we've run out of route components, render the last route map as a directory page

    None
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
            ("sub-route".to_string(), Route::SubRouteMap(sub_route_map)),
        ]);

        route_map
    }

    #[test]
    fn load_top_level_route() {
        let route_map = create_route_map();
        let page = load_route("/index", &route_map);
        assert_eq!(page, Some("index-html".to_string()));
    }

    #[test]
    fn load_index() {
        let route_map = create_route_map();
        let page = load_route("/", &route_map);
        assert_eq!(page, Some("index-html".to_string()));
    }

    #[test]
    fn load_sub_route() {
        let route_map = create_route_map();
        let page = load_route("/sub-route/sub-page", &route_map);
        assert_eq!(page, Some("sub-page-html".to_string()));
    }

    #[test]
    fn load_dead_route() {
        let route_map = create_route_map();
        let page = load_route("null", &route_map);
        assert_eq!(page, None);
    }
}
