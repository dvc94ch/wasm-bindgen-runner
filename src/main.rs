use std::io::{stdout, stderr, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let path_arg = std::env::args().nth(1).unwrap();
    match Task::from(path_arg.as_str()) {
        Task::Test => {
            let output = Command::new("wasm-bindgen-test-runner")
                .arg(path_arg)
                .output()
                .expect("Failed to run wasm-bindgen-test-runner");
            stdout().write_all(&output.stdout).unwrap();
            stderr().write_all(&output.stderr).unwrap();
        },
        Task::Run { mut out_dir, wasm } => {
            out_dir.push("wasm-bindgen");
            let output = Command::new("wasm-bindgen")
                .arg(path_arg)
                .arg("--out-dir")
                .arg(out_dir.to_str().unwrap())
                .arg("--no-typescript")
                .arg("--no-modules")
                .arg("--browser")
                .output()
                .expect("Failed to run wasm-bindgen");
            stdout().write_all(&output.stdout).unwrap();
            stderr().write_all(&output.stderr).unwrap();
            write_template(out_dir.clone(), wasm);
            println!("Running at http://127.0.0.1:4000");
            Command::new("basic-http-server")
                .arg(out_dir.to_str().unwrap())
                .output()
                .expect("Failed to run basic-http-server");
        }
    }
}

fn write_template(mut out_dir: PathBuf, mut wasm: String) {
    out_dir.push("index.html");
    let mut js = wasm.clone();
    js.push_str(".js");
    wasm.push_str("_bg.wasm");
    let html = format!("
<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    <title>wasm-bindgen-runner</title>
  </head>
  <body style=\"margin: 0; padding: 0; width: 100%; height: 100%;\">
    <div id=\"rust-web-app\" style=\"width: 100%; height: 100%;\"></div>

    <script src=\"{}\"></script>
    <script>window.wasm_bindgen(`{}`)</script>
  </body>
</html>
", js, wasm);
    std::fs::write(out_dir, html).unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Task {
    Run { out_dir: PathBuf, wasm: String },
    Test,
}

impl From<&str> for Task {
    fn from(path: &str) -> Self {
        let mut path_buf = PathBuf::from(path);
        let wasm = path_buf.file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        path_buf.pop();
        match path_buf.file_name().unwrap().to_str().unwrap() {
            "deps" => Task::Test,
            _ => Task::Run { out_dir: path_buf, wasm },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_run() {
        let path = "target/wasm32-unknown-unknown/debug/libweb.wasm";
        assert_eq!(Task::from(path), Task::Run);
    }

    #[test]
    fn test_task_test() {
        let path = "target/wasm32-unknown-unknown/debug/deps/libweb.wasm";
        assert_eq!(Task::from(path), Task::Test);
    }
}
