use std::{collections::HashMap, error::Error, fs, ops::Add};

use crate::router::{Route, RouteMap};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Represents the state of the site
#[derive(Debug)]
pub struct Site {
    pub top_level_routes: Vec<String>,
    pub route_map: RouteMap,
}

impl Site {
    pub fn load_from_disk(path: &str) -> Result<Self> {
        let route_map = Self::load_routes_from_disk(path)?;
        let top_level_routes = route_map.keys().cloned().collect();
        Ok(Site {
            top_level_routes,
            route_map,
        })
    }
    /// Recursively loads site markup from a specified directory. Directory structure is expanded as follows:
    /// ```text
    /// + site
    /// 	+ posts
    /// 		+ first-post.md
    /// 	+ about.md
    /// ```
    /// becomes:
    /// ```text
    /// RouteMap
    ///		'about' ->
    /// 		[html render of about.md]
    /// 	'posts' ->
    /// 		RouteMap
    /// 			'first-post' ->
    /// 				[html render of first-post.md]
    ///
    /// ```
    ///
    fn load_routes_from_disk(path: &str) -> Result<RouteMap> {
        let mut route_map: RouteMap = HashMap::new();

        // Sort by creation date?
        for entry in fs::read_dir(path)? {
            let item_path = entry?.path();
            let route = item_path.file_stem().and_then(|p| p.to_str()).unwrap();

            let metadata = fs::metadata(&item_path)?;

            // If the resource is a directory, recursively load sub routes.
            // Not that example.com/posts/
            match metadata.is_dir() {
                true => {
                    let sub_path = String::from(path).add("/").add(route);
                    if let Ok(sub_routes) = Self::load_routes_from_disk(&sub_path) {
                        route_map.insert(route.to_string(), Route::SubRouteMap(sub_routes));
                    }
                }
                false => {
                    let ext = item_path.extension().and_then(|s| s.to_str());

                    if ext == Some("md") {
                        let markdown = fs::read_to_string(&item_path)?;
                        let html = markdown::to_html(&markdown);
                        route_map.insert(route.to_string(), Route::Page(html));
                    }
                }
            }
        }

        Ok(route_map)
    }
}
