use applications::prelude::*;
use std::thread;

struct Context {
    apps: Vec<String>,
}

fn get_all_apps() -> Vec<String> {
    vec!["App1".to_string(), "App2".to_string()]
}

impl Context {
    fn new() -> Self {
        Self { apps: vec![] }
    }

    fn refresh_apps(&mut self) -> Result<()> {
        self.apps = vec!["App1".to_string(), "App2".to_string()];
        Ok(())
    }

    fn get_all_apps(&self) -> Vec<String> {
        thread::sleep(tokio::time::Duration::from_secs(3));
        get_all_apps()
    }
}

fn main() {
    let ctx = Context::new();
    let apps = ctx.get_all_apps();
    println!("{:?}", apps)
}
