use std::fs;

use crate::{output_path, PostMCTSHookArgs, PostMCTSHookFn};

pub fn graph_hook() -> PostMCTSHookFn {
    Box::new(
        |PostMCTSHookArgs {
             run,
             turn,
             agent,
             mcts,
             ..
         }| {
            fs::create_dir_all(format!(
                "{}/{}/graphs/agent{}/",
                output_path(),
                run.map(|n| n.to_string()).unwrap_or_default(),
                agent.0
            ))
            .unwrap();
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(format!(
                    "{}/{}/graphs/agent{}/turn{:06}.dot",
                    output_path(),
                    run.map(|n| n.to_string()).unwrap_or_default(),
                    agent.0,
                    turn
                ))
                .unwrap();

            dot::render(mcts, &mut file).unwrap();
        },
    )
}
