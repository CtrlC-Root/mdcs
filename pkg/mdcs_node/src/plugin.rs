use std::error::Error;
use std::process::{Command, Child, ExitStatus};

use crate::node::PluginConfig;

#[derive(Debug)]
pub struct Plugin {
    pub name: String,
    pub config: PluginConfig
}

#[derive(Debug)]
pub struct PluginInstance {
    pub process: Child
}

impl Plugin {
    pub fn spawn(&self) -> Result<PluginInstance, Box<dyn Error>> {
        let process = Command::new(&self.config.command[..])
            .spawn()?;

        Ok(PluginInstance {
            process
        })
    }
}

impl PluginInstance {
    pub fn is_running(&mut self) -> bool {
        self.process.try_wait().unwrap().is_none()
    }

    pub fn stop(&mut self) -> Result<ExitStatus, Box<dyn Error>> {
        if self.is_running() {
            // TODO: ask it nicely instead
            self.process.kill()?;
        }

        // wait for it to quit and return the exit status
        let status =  self.process.wait()?;
        Ok(status)
    }
}
