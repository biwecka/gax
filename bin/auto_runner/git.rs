// Imports /////////////////////////////////////////////////////////////////////
use git2::{
    Cred, IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature,
};

use crate::{env::Env, error::Error};

// Git /////////////////////////////////////////////////////////////////////////
/// This struct provides the functionality of executing a limited set of
/// git commands for managing a git repository.
pub struct Git {
    repo: Repository,
    username: String,
    password: String,
}

#[allow(unused)]
impl Git {
    /// Open a git repository located at the `plots_repo` variable provided
    /// by [`Env`].
    pub fn open_repo(env: &Env) -> Result<Git, Error> {
        let repo = Repository::open(env.plots_repo.clone())?;
        let username = env.git_username.clone();
        let password = env.git_password.clone();

        Ok(Self { repo, username, password })
    }

    /// Execute `git fetch origin main`.
    pub fn fetch(&self) -> Result<(), Error> {
        self.repo.find_remote("origin")?.fetch(&["main"], None, None)?;

        Ok(())
    }

    /// Execute a git rebase command.
    pub fn rebase(&self) -> Result<(), Error> {
        // Prepare rebase
        let local_commit = {
            let local_branch =
                self.repo.find_branch("main", git2::BranchType::Local)?;

            self.repo.reference_to_annotated_commit(local_branch.get())?
        };

        let fetch_head_commit = {
            let fetch_head = self.repo.find_reference("FETCH_HEAD")?;

            self.repo.reference_to_annotated_commit(&fetch_head)?
        };

        // Create rebase operation
        let mut rebase = self.repo.rebase(
            Some(&local_commit),
            Some(&fetch_head_commit),
            None,
            None,
        )?;

        // Perform rebase
        while let Some(op) = rebase.next() {
            op?;

            rebase.commit(
                None,
                &self.repo.signature().expect("Repo Signature"),
                Some(&format!("Rebased: {}", "main")),
            )?;
        }

        rebase.finish(None)?;

        Ok(())
    }

    /// Execute `git add .`.
    pub fn add_all(&self) -> Result<(), Error> {
        let mut staging_area = self.repo.index()?;
        staging_area.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;

        // Actually stage the changes
        staging_area.write()?;

        Ok(())
    }

    /// Execute `git commit -m "Add data"`.
    pub fn commit(&self) -> Result<(), Error> {
        // Get the staging area (index).
        let mut index = self.repo.index()?;

        // Write the index (staging area) to the repository tree
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        // Get the current branch (HEAD)
        let head = self.repo.head()?;
        let parent_commit = head.peel_to_commit()?; // Get the latest commit as the parent

        // Create a signature for the author and committer
        let signature = Signature::now("Auto Runner", "dummy@example.com")?;

        // The commit message
        let message = "Add data";

        self.repo.commit(
            Some("HEAD"), // Point HEAD to the new commit
            &signature,   // The author of the commit
            &signature,   // The committer of the commit
            message,      // Commit message
            &tree,        // Tree object representing the staged changes
            &[&parent_commit], // Parent commits (for a normal commit,
                          // there’s only one parent)
        )?;

        Ok(())
    }

    /// Execute `git push origin main`.
    pub fn push(&self) -> Result<(), Error> {
        // Get remote
        let mut remote = self.repo.find_remote("origin")?;

        // Setup authentication
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            Cred::userpass_plaintext(&self.username, &self.password)
        });

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);

        // Push
        remote.push(&["refs/heads/main"], Some(&mut push_options))?;

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
