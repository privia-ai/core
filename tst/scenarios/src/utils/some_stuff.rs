use std::path::Path;

fn deploy_canisters(root_dir: &str) {
    use std::process::Command;
    let scripts_dir = Path::new(root_dir).join("scripts/test_env/");

    let ledger_output = Command::new(scripts_dir.join("deploy_ledger.sh"))
        .current_dir(root_dir)
        .output()
        .unwrap();

    println!("Text: {}", String::from_utf8_lossy(&ledger_output.stdout));
}