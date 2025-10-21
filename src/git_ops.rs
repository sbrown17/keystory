use anyhow::Result;
use git2::{Repository, Signature};

pub fn commit_change(path: &str, message: &str) -> Result<()> {
    let repo = Repository::open(".")?;

    let mut index = repo.index()?;
    index.add_path(std::path::Path::new(path))?;
    index.write()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let sig = Signature::now("KeyStory", "keystory@app")?;

    let parent_commit = match repo.head() {
        Ok(head) => head.peel_to_commit()?,
        Err(_) => {
            // First commit in repo
            return repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                message,
                &tree,
                &[],
            )
            .map(|_| ());
        }
    };

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        message,
        &tree,
        &[&parent_commit],
    )?;
    Ok(())
}
