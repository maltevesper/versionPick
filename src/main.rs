use version_pick::{Git, VersionControlSystem};

fn main() {
    let repo = Git::from_url("https://github.com/maltevesper/darglint.git");

    for (rev, _oid) in repo.heads().unwrap() {
        println!("Ok we got {}", rev);
    }
}
