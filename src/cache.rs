use super::models::Repository;

use crate::models::Tag;
use std::io::Write;

const CACHE_DIR: &str = "./cache";

pub fn find_or_initialize(repository_url: &str) -> Result<Repository, Box<dyn std::error::Error>> {
    let url = url::Url::parse(&format!("https://{}", repository_url))?;
    let host = url.host_str().unwrap();
    let path = url.path();
    let filepath = format!("{}/{}", CACHE_DIR, filename(host, path));
    if !std::path::Path::new(&filepath).exists() {
        return Ok(build_repository(&host, &path));
    }
    let file = std::fs::File::open(&filepath)?;
    let repository: Repository = serde_json::from_reader(file)?;
    Ok(repository)
}

pub fn save(repository: &Repository) -> Result<(), Box<dyn std::error::Error>> {
    let filepath = format!(
        "{}/{}",
        CACHE_DIR,
        filename(&repository.host, &repository.path)
    );
    let mut file = std::fs::File::create(filepath)?;
    let json = serde_json::to_string(&repository)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn update_tag(
    repository_url: &str,
    target_tag: &Tag,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut repository: Repository = find_or_initialize(repository_url)?;
    let mut is_exists = false;
    let mut updated_tags: Vec<Tag> = repository
        .tags
        .into_iter()
        .map(|tag| {
            if tag.name == target_tag.name {
                is_exists = true;
                target_tag.clone()
            } else {
                tag
            }
        })
        .collect();
    if !is_exists {
        updated_tags.push(target_tag.clone());
    }
    repository.tags = updated_tags;
    save(&repository)?;
    Ok(())
}

fn build_repository(host: &str, path: &str) -> Repository {
    Repository {
        host: host.to_string(),
        path: path.to_string(),
        tags: vec![],
    }
}

fn filename(host: &str, path: &str) -> String {
    format!("{}{}.json", safe_filename(host), safe_filename(path))
}

fn safe_filename(string: &str) -> String {
    string.replace("/", "_").replace(":", "_").replace(".", "_")
}

#[cfg(test)]
mod tests {
    use crate::cache;
    use crate::models::Tag;

    #[test]
    fn it_works() {
        let mut repository =
            cache::find_or_initialize("https://registry.hub.docker.com/library/alpine").unwrap();
        repository.tags = vec![
            Tag {
                name: "latest".to_string(),
                digest: None,
                is_synced: false,
            },
            Tag {
                name: "1.0.0".to_string(),
                digest: None,
                is_synced: false,
            },
        ];
        let result = cache::save(&repository);
        assert!(result.is_ok());
    }
}
