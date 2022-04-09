use argh::FromArgs;
use std::process::Command;
use std::process;
use regex::Regex;

#[derive(FromArgs)]
/// オプション
pub struct Options {
    /// create pull request, what branch?
    #[argh(switch, short = 'p')]
    pull_request: bool,

    /// specify base branch
    #[argh(option, short = 'b', default = "String::from(\"main\")")]
    base_branch: String
}

async fn get_http(url: &str) -> reqwest::StatusCode {
    let body = reqwest::get(url).await.unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    }).status();

    body
}

#[tokio::main]
async fn main() {
    let options: Options = argh::from_env();

    let git_remote_url_cmd = Command::new("git").arg("remote").arg("-v").output().expect("failed to run. `git remote`");
    
    if String::from_utf8(git_remote_url_cmd.stderr).unwrap() != "" {
        eprintln!("not a git repo");
		process::exit(1);
    }

    let git_remote_url = String::from_utf8(git_remote_url_cmd.stdout).unwrap();
    let re = Regex::new(r"github.com:(.*).git").unwrap();
    let git_remote_url_caps = re.captures(&git_remote_url).unwrap();

    let current_branch_cmd = Command::new("git").arg("branch").arg("--contains").output().expect("failed to run. `git branch --contains`");
    let current_branch = String::from_utf8(current_branch_cmd.stdout).unwrap();
    let re = Regex::new(r"\* (.*)").unwrap();
    let current_branch_caps = re.captures(&current_branch).unwrap();

    if options.pull_request {
        let branch_name = options.base_branch;
        let url = "https://github.com/".to_string() + &git_remote_url_caps[1] + "/compare/" + &branch_name + "..." + &current_branch_caps[1];
        open::that(url).unwrap();
        process::exit(0);
    }

    let base_url = "https://github.com/".to_string() + &git_remote_url_caps[1];
    // let url = base_url + "/tree/" + &current_branch_caps[1];
    let url = base_url + "/tree/" + &current_branch_caps[1];

    let status = get_http(&url).await == 404;
    if status {
        open::that("https://github.com/".to_string() + &git_remote_url_caps[1]).unwrap();
        process::exit(0);
    }

    open::that(url).unwrap();
}
