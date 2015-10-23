use std::process::Command;

fn main() {
    println!("使用 wait 方式");
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("cd /tmp && ls -lht")
        .spawn()
        //.unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });
        .ok()
        .expect("exec fail.");
    let ecode = child.wait()
        .unwrap_or_else(|e| { panic!("failed to wait on child: {}", e) });
    println!("ecode: {}", ecode);

    println!("\n使用输出缓冲");
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo \"just test, wow\"")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
