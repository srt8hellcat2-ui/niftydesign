//! Links for this unofficial NiftyDesign build.

use serde_json::{Value, json};

/// This unofficial build's source repository.
pub const GITHUB: &str = "https://github.com/srt8hellcat2-ui/niftydesign";
/// This build's issue tracker.
pub const ISSUES: &str = "https://github.com/srt8hellcat2-ui/niftydesign/issues";

/// All links as JSON (the `app.links` command).
pub fn all() -> Value {
    json!({
        "github": GITHUB,
        "issues": ISSUES,
    })
}

/// A link by key (`github`, `issues`).
pub fn get(key: &str) -> Option<&'static str> {
    Some(match key {
        "github" => GITHUB,
        "issues" => ISSUES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_app_slug() {
        assert!(GITHUB.ends_with("/srt8hellcat2-ui/niftydesign"));
        assert!(ISSUES.starts_with(GITHUB));
        for k in ["github", "issues"] {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
    }
}
