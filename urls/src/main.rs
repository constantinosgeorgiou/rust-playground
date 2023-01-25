use url::{ParseError, Url};

fn main() -> Result<(), ParseError> {
    let full_url = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let url_path = Url::parse(full_url)?;
    println!("The path part of the URL is: {}", url_path.path());

    let url_base = base_url(url_path)?;
    println!("The base of the URL is: {}", url_base);

    let github_path = "/rust-lang/cargo";
    let github = build_github_url(github_path)?;
    println!("Github URL: {}", github);

    Ok(())
}

fn base_url(mut url: Url) -> Result<Url, ParseError> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(ParseError::RelativeUrlWithoutBase);
        }
    }
    url.set_query(None);

    Ok(url)
}

fn build_github_url(path: &str) -> Result<Url, ParseError> {
    const GITHUB: &'static str = "https://github.com";

    let base = Url::parse(GITHUB).expect("hardcoded URL is known to be valid");
    let joined = base.join(path)?;

    Ok(joined)
}
