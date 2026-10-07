use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;

#[test]
fn accepts_empty_input() {
    let output = Command::new(env!("CARGO_BIN_EXE_ascii-filter"))
        .stdin(Stdio::null())
        .output()
        .expect("failed to run ascii-filter");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn keeps_only_ascii_bytes() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ascii-filter"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run ascii-filter");

    child
        .stdin
        .take()
        .expect("missing standard input")
        .write_all(&(0..=255).collect::<Vec<u8>>())
        .expect("failed to write input");

    let output = child.wait_with_output().expect("failed to wait for filter");
    assert!(output.status.success());
    assert_eq!(output.stdout, (0..=127).collect::<Vec<u8>>());
    assert!(output.stderr.is_empty());
}

#[test]
fn filters_a_large_stream() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ascii-filter"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run ascii-filter");
    let mut input = child.stdin.take().expect("missing standard input");
    let writer = thread::spawn(move || {
        let bytes = (0..=255).collect::<Vec<u8>>();
        for _ in 0..8192 {
            input.write_all(&bytes).expect("failed to write input");
        }
    });

    let output = child.wait_with_output().expect("failed to wait for filter");
    writer.join().expect("input writer failed");
    assert!(output.status.success());
    assert_eq!(output.stdout, (0..=127).collect::<Vec<u8>>().repeat(8192));
    assert!(output.stderr.is_empty());
}
