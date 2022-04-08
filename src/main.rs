use argh::FromArgs;
use std::process::Command;
use std::process;
use regex::Regex;

#[derive(FromArgs)]
/// オプション
pub struct Options {
    /// create pull request?
    #[argh(switch, short = 'p')]
    pull_request: bool,
}

fn main() {
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

    let url = "https://github.com/".to_string() + &git_remote_url_caps[1] + "/compare/develop..." + &current_branch_caps[1];
    println!("{}", url);
    
    // println!("{}", &caps[1]);

    // 引数を受け取る
    // 現在いるディレクトリが gitリポジトリかどうかチェック
        // gitリポジトリじゃなければ、違う旨を表示して終了
        // gitリポジトリなら、次へ
    // git remote をとってくる
        // git remote -v
            // origin  git@github.com:timeleap-rura/rura-platform-web.git (fetch)
            // origin  git@github.com:timeleap-rura/rura-platform-web.git (push)
        // grep fetch
    // 現在いるブランチをとってくる（git command を実行）
    // 受け取った引数に基づいて URL を生成する
        // -p が有効なら
            // PR作成ページのURLを生成
        // -p が無効なら
            // ブランチのページのURLを生成
    println!("{:?}", options.pull_request);
}
