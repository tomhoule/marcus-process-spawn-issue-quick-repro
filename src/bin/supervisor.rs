use std::process;

fn main() {
    let mut child = process::Command::new("./script.sh")
        .stdout(process::Stdio::piped())
        .spawn()
        .unwrap();

    // let mut child = process::Command::new("cargo")
    //     .arg("run")
    //     .arg("--bin=child")
    //     .spawn()
    //     .unwrap();

    println!("child process is spawned — pid is {}", child.id());
    std::thread::sleep(std::time::Duration::from_millis(5000));

    println!("killing the child process now");
    child.kill().unwrap();

    drop(child);
    // let mut stdout = child.stdout.unwrap();
    // drop(stdout);

    std::thread::sleep(std::time::Duration::from_secs(3600));
}
