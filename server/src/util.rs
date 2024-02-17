use url::Url;

pub fn base_url(mut url: Url) -> Option<Url> {
	match url.path_segments_mut() {
		Ok(mut path) => {
			path.clear();
		}
		Err(_) => return None,
	}

	url.set_query(None);

	Some(url)
}
