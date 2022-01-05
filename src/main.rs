use semver::Version;

fn main() {
    let value = reqwest::blocking::get(format!("https://www.python.org/ftp/python/").as_str())
        .unwrap()
        .text()
        .unwrap();
    let mut versions = vec![];
    let doc = scraper::Html::parse_document(&value);
    let sel = scraper::Selector::parse("a").unwrap();
    for (index, node) in doc.select(&sel).enumerate() {
        if node.inner_html().is_empty() || index == 0 {
            continue;
        }
        let mut version = node.inner_html();
        version.retain(|c| c != '/');
        match Version::parse(&version.to_string()) {
            Ok(v) => versions.push(v),
            _ => continue,
        }
    }
    versions.sort();
    for ver in &versions {
        let tar_url = format!(
            "https://www.python.org/ftp/python/{}/Python-{}.tar.xz",
            ver, ver
        );
        println!("{} - {}", ver, tar_url)
    }
}