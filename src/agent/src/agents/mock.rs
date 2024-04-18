use crate::agents::Agent;
use crate::{AgentResult, workload};

pub struct MockAgent {
    workload_config: workload::config::Config,
}

impl From<workload::config::Config> for MockAgent {
    fn from(workload_config: workload::config::Config) -> Self {
        Self {
            workload_config,
        }
    }
}

impl Agent for MockAgent {
    fn prepare(&self) -> AgentResult<()> {
        todo!()
    }

    fn run(&self) -> AgentResult<()> {
        todo!()
    }
}
