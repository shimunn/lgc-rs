use last_git_commit::LastGitCommit;

fn main() {

    let lgc = LastGitCommit::new().build().unwrap();
    let long = lgc.id().long();
    let short = lgc.id().short();
    let range = lgc.id().range(0..3).unwrap();

    println!("Long: {}", long); // "c4f94258c12b8905f3d57f879ae1171ce367cd29"
    println!("Short: {}", short); // "c4f9425"
    println!("Range: {}", range); // "c4f"

}
