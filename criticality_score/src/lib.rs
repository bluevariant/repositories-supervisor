use regex::Regex;

pub trait Repository {
    fn name(&self) -> String;
    fn url(&self) -> String;
    fn language(&self) -> String;
    fn last_commit(&self) -> String;
    fn created_since(&self) -> i32;
    fn updated_since(&self) -> i32;
    fn contributor_count(&self) -> i32;
    fn org_count(&self) -> i32;
    fn commit_frequency(&self) -> i32;
    fn recent_releases_count(&self) -> i32;
    fn updated_issues_count(&self) -> i32;
    fn closed_issues_count(&self) -> i32;
    fn comment_frequency(&self) -> i32;
    fn dependents_count(&self) -> i32;
}

#[derive(Debug)]
pub struct GithubRepository {}

impl Repository for GithubRepository {
    fn name(&self) -> String {
        String::from("OK")
    }

    fn url(&self) -> String {
        todo!()
    }

    fn language(&self) -> String {
        todo!()
    }

    fn last_commit(&self) -> String {
        todo!()
    }

    fn created_since(&self) -> i32 {
        todo!()
    }

    fn updated_since(&self) -> i32 {
        todo!()
    }

    fn contributor_count(&self) -> i32 {
        todo!()
    }

    fn org_count(&self) -> i32 {
        todo!()
    }

    fn commit_frequency(&self) -> i32 {
        todo!()
    }

    fn recent_releases_count(&self) -> i32 {
        todo!()
    }

    fn updated_issues_count(&self) -> i32 {
        todo!()
    }

    fn closed_issues_count(&self) -> i32 {
        todo!()
    }

    fn comment_frequency(&self) -> i32 {
        todo!()
    }

    fn dependents_count(&self) -> i32 {
        todo!()
    }
}

pub fn get_github_repo(url: impl Into<String>) {
    let url = url.into();
    let re = Regex::new(
        r"(^(https|http)://|^)github.com/(?P<owner>[^@\s!/]+)/(?P<repository>[^@\s!/#?]+)",
    )
    .unwrap();
    let captures = re.captures(url.as_str()).unwrap();

    if captures.name("owner").is_some() && captures.name("repository").is_some() {
        let owner = captures.name("owner").unwrap().as_str();
        let repository = captures.name("repository").unwrap().as_str();

        println!("{}/{}", owner, repository);
    }
}
