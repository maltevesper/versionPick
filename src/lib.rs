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
    type BranchId = String;

    /**
     * See https://github.com/rust-lang/rust/issues/29661 -> we can not typedef based on trait types inside a trait.
     * => ignore the complexity here.
     */
    #[allow(clippy::type_complexity)]
    fn heads(&self) -> Result<Vec<(Self::BranchId, Self::RevisionId)>, Error> {
        let mut remote = Remote::create_detached(&self.url)?;
        remote.connect(Direction::Fetch)?;

        Ok(remote
            .list()?
            .iter()
            .map(|head| (head.name().to_string(), head.oid()))
            .collect())
    }
}
