use tempdir::TempDir;

fn main() {
    // Create a temporary directory to download the apps to
    let temp_dir = TempDir::new("gitoxide").unwrap();
    let temp_dir_path = temp_dir.path();
    let mut prepare = git_repository::prepare_clone(
        "https://github.com/AaronDewes/gitoxide-bug-demo",
        temp_dir_path,
    ).expect("Failed to prepare");
    prepare.fetch_only(
        git_repository::progress::Discard,
        &std::sync::atomic::AtomicBool::default(),
    ).expect("Failed to fetch");
    println!("Downloaded to {}", temp_dir_path.display());
}
