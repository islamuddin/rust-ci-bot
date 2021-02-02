use chrono::Utc;
use job_scheduler::{Job, JobScheduler};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("{:?}", execute_bot());
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn execute_bot() -> std::io::Result<String> {
    std::fs::remove_file("foo.txt")?;
    std::fs::remove_file(".git/index.lock");
    std::fs::remove_file("./.git/index.lock");
    std::fs::remove_file(".git/.COMMIT_EDITMSG.swp");
    let mut file = File::create("foo.txt")?;
    let now = Utc::now();
    file.write_all(format!("updated at {:?}", now).as_bytes())?;
    Command::new("git").args(&["add", "foo.txt"]).spawn()?;
    Command::new("git")
        .args(&["commit", "-a", "-m", format!("run yaw {:?}", now).as_str()])
        .spawn()?;
    Ok(format!(
        "{:?}",
        Command::new("git")
            .args(&["push", "origin", "main"])
            .output()
    ))
}
