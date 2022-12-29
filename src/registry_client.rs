use crate::models;
use crate::models::Tag;
use models::Repository;
use serde_json::from_str;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

const THREADS: i32 = 10;

pub fn get_tags(repository: &Repository) -> Result<Vec<Tag>, Box<dyn std::error::Error>> {
    let tags_no_digest = get_tags_with_no_digest(repository)?;
    let mut handles = vec![];
    let tags: Arc<Mutex<Vec<Tag>>> = Arc::new(Mutex::new(vec![]));
    let chunk_size = if tags_no_digest.len() < THREADS as usize {
        tags_no_digest.len()
    } else {
        tags_no_digest.len() / THREADS as usize
    };
    let chunks: Vec<Vec<String>> = tags_no_digest
        .chunks(chunk_size)
        .map(|x| x.to_vec())
        .collect();
    for chunk in chunks {
        let tags = Arc::clone(&tags);
        let repository_url = format!("{}{}", repository.host, repository.path);
        let handle = thread::spawn(move || {
            for tag in chunk {
                let output = Command::new("skopeo")
                    .arg("inspect")
                    .arg("--raw")
                    .arg(format!("docker://{}:{}", &repository_url, tag))
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to get digest");
                let output = Command::new("base64")
                    .stdin(Stdio::from(output.stdout.unwrap()))
                    .stdout(Stdio::piped())
                    .output()
                    .expect("failed to get tag");

                if output.status.code() != Some(0) {
                    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                } else {
                    tags.lock().unwrap().push(Tag {
                        name: tag.to_string(),
                        digest: Some(String::from_utf8_lossy(&output.stdout).to_string()),
                        is_synced: false,
                    });
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let result = tags.lock().unwrap().to_vec();
    Ok(result)
}
fn get_tags_with_no_digest(
    repository: &Repository,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("skopeo")
        .arg("list-tags")
        .arg(format!("docker://{}{}", repository.host, repository.path))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed skopeo list-tags:");

    let output = Command::new("jq")
        .arg(".Tags")
        .stdin(Stdio::from(output.stdout.unwrap()))
        .stdout(Stdio::piped())
        .output()
        .expect("failed to get tags");

    if output.status.code() != Some(0) {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return Err("failed to get tags".into());
    }
    let tags: Vec<String> = from_str(&String::from_utf8_lossy(&output.stdout).to_string())
        .expect("failed to parse tags");
    Ok(tags)
}

#[cfg(test)]
mod tests {
    use crate::models::Repository;
    use crate::registry_client::{get_tags, get_tags_with_no_digest};

    #[test]
    fn it_works_get_tags_with_no_digest() {
        let repository = Repository {
            host: "docker.io".to_string(),
            path: "/ogontaro/hello-world".to_string(),
            tags: vec![],
        };

        let result = get_tags_with_no_digest(&repository);
        assert!(result.is_ok());
    }
    #[test]
    fn it_works_get_tags() {
        let repository = Repository {
            host: "docker.io".to_string(),
            path: "/ogontaro/hello-world".to_string(),
            tags: vec![],
        };

        let result = get_tags(&repository);
        assert!(result.is_ok());
    }
}
