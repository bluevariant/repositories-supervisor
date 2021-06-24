use criticality_score::{get_github_repo, GithubRepository, Repository};

fn main() {
    let github_repository = GithubRepository {};

    get_github_repo("https://github.com/XAMPPRocky/octocrab");
}
