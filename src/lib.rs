use fail::fail_point;
use failure::Error;
use git2::{Direction, Oid, Remote};

pub trait VersionControlSystem {
    type RevisionId;
    type BranchId;

    /**
     * Since "associated type defaults are unstable" (see https://github.com/rust-lang/rust/issues/29661),
     * we can not define a type like at the end of this comment, without using nightly rust. Therefore,
     * we ignore type_complexity.
     *
     * type BranchHeadVector = Vec<(Self::BranchId, Self::RevisionId)>;
     */
    #[allow(clippy::type_complexity)]
    fn heads(&self) -> Result<Vec<(Self::BranchId, Self::RevisionId)>, Error>;
}

pub struct Git {
    url: String,
}

impl Git {
    pub fn from_url(url: &str) -> Git {
        Git {
            url: url.to_string(),
        }
    }
}

impl VersionControlSystem for Git {
    type RevisionId = Oid;
    type BranchId = String; //TODO: can we change the code to use &str?

    /**
     * See https://github.com/rust-lang/rust/issues/29661 -> we can not typedef based on trait types inside a trait.
     * => ignore the complexity here.
     */
    #[allow(clippy::type_complexity)]
    fn heads(&self) -> Result<Vec<(Self::BranchId, Self::RevisionId)>, Error> {
        fail_point!("git.heads.create_detached", |_| {
            Err(git2::Error::from_str("Injected create_detach error.").into())
        });
        let mut remote = Remote::create_detached(&self.url)?;
        remote.connect(Direction::Fetch)?;

        Ok(remote
            .list()?
            .iter()
            .map(|head| (head.name().to_string(), head.oid()))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cmd_lib::run_fun;
    use mktemp::Temp;
    use std::io::Error;

    #[test]
    fn test_heads() -> Result<(), Error> {
        let temp_dir = Temp::new_dir()?;
        let temp_path = temp_dir.to_str().unwrap();
        println!(
            "{:?}",
            run_fun! (
                cd ${temp_path};
                git init --initial-branch=main;
                git commit --allow-empty -m "commit 1";
                git commit --allow-empty -m "commit 2";
                git switch -c feature/branch;
                git commit --allow-empty -m "feature commit";
                git log --all --decorate --oneline;
            )?
        );
        let mut heads = ["HEAD", "refs/heads/feature/branch", "refs/heads/main"]
            .into_iter()
            .map(|revision: &str| {
                (
                    revision.to_string(),
                    Oid::from_str(&run_fun! (cd ${temp_path}; git rev-parse ${revision}).unwrap())
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();
        heads.sort();
        let git = Git::from_url(temp_path);
        let mut git_heads = git.heads().unwrap();
        git_heads.sort();
        assert_eq!(git_heads, heads);
        Ok(())
    }
}
