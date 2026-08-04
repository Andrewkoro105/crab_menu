use tokio::process::Command;
use tracing::{error, info};

use crate::app::functions::run::Run;

#[derive(Clone, Hash)]
pub struct Shell {
    pub shell: String,
    pub args: Vec<String>,
    pub script: String,
    pub close_it: bool,
}

impl Run for Shell {
    async fn run(&self) {
        let result = Command::new(self.shell.clone()).args(self.args.clone()).arg(self.script.clone()).output().await;
        match result {
            Ok(output) => info!("Run output: {output:?}"),
            Err(err) => error!("Run error: {err}"),
        }
    }
    
    fn close_id(&self) -> bool {
        self.close_it
    }
}