use argh::FromArgs;
use std::process::Command;

#[derive(FromArgs)]
/// オプション
pub struct Options {
    /// create pull request?
    #[argh(switch, short = 'p')]
    pull_request: bool,
}

fn main() {
    let options: Options = argh::from_env();

    let output = Command::new("git").arg("remote").arg("--version").output().expect("failed to run. `git remote`");
    println!("{}", String::from_utf8_lossy(&output.stdout));

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
