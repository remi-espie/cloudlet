use super::AgentOutput;
use crate::agents::Agent;
use crate::{workload, AgentResult};
use std::fs::create_dir_all;
use std::time::SystemTime;

pub struct SimpleAgent {
    workload_config: workload::config::Config,
}

impl From<workload::config::Config> for SimpleAgent {
    fn from(workload_config: workload::config::Config) -> Self {
        Self { workload_config }
    }
}

impl Agent for SimpleAgent {
    fn prepare(&self) -> AgentResult<AgentOutput> {
        let dir = format!("/tmp/{}", self.workload_config.workload_name);

        println!("Function directory: {}", dir);

        create_dir_all(&dir).expect("Unable to create directory");

        std::fs::write(
            format!("{}/simple.txt", &dir),
            format!(
                "Simple agent for {} - written at {:?}",
                self.workload_config.workload_name,
                SystemTime::now(),
            ),
        )
        .expect("Unable to write main.rs file");

        Ok(AgentOutput {
            exit_code: 0,
            stdout: "Build successfully!".into(),
            stderr: String::default(),
        })
    }

    fn run(&self) -> AgentResult<AgentOutput> {
        let dir = format!("/tmp/{}", self.workload_config.workload_name);

        let content = std::fs::read_to_string(format!("{}/simple.txt", &dir))
            .expect("Unable to read mock.txt file");

        std::fs::remove_dir_all(dir).expect("Unable to remove directory");

        Ok(AgentOutput {
            exit_code: 0,
            stdout: content,
            stderr: String::default(),
        })
    }
}
