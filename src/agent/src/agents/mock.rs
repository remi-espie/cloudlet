use crate::agents::Agent;
use crate::{workload, AgentResult};
use std::fs::create_dir_all;
use std::time::SystemTime;

pub struct MockAgent {
    workload_config: workload::config::Config,
}

impl From<workload::config::Config> for MockAgent {
    fn from(workload_config: workload::config::Config) -> Self {
        Self { workload_config }
    }
}

impl Agent for MockAgent {
    fn prepare(&self) -> AgentResult<()> {
        let dir = format!("/tmp/{}", self.workload_config.workload_name);

        println!("Function directory: {}", dir);

        create_dir_all(&dir).expect("Unable to create directory");

        std::fs::write(
            format!("{}/mock.txt", &dir),
            format!(
                "Mock agent for {} - written at {:?}",
                self.workload_config.workload_name,
                SystemTime::now()
            ),
        )
        .expect("Unable to write main.rs file");

        Ok(())
    }

    fn run(&self) -> AgentResult<()> {
        let dir = format!("/tmp/{}", self.workload_config.workload_name);

        let content = std::fs::read_to_string(format!("{}/mock.txt", &dir))
            .expect("Unable to read mock.txt file");

        println!("Mock agent content: {}", content);

        Ok(())
    }
}
