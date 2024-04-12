use url::Url;

/// Returns the base URL of the provided `url`.
///
/// # Examples
/// use crate::util::base_url;
/// use url::Url;
///
/// let base_url = base_url(Url::parse("http://example.com/some/url"));
/// assert!(base_url, "http://example.com");
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
