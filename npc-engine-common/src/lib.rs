/*
 *  SPDX-License-Identifier: Apache-2.0 OR MIT
 *  © 2020-2022 ETH Zurich and other contributors, see AUTHORS.txt for details
 */

//! This is the core of the NPC engine, containing the MCTS algorithm implementation and related abstractions.

mod active_task;
mod behavior;
mod config;
mod domain;
mod edge;
mod mcts;
mod node;
mod state_diff;
mod task;
mod util;

pub use active_task::*;
pub use behavior::*;
pub use config::*;
pub use domain::*;
pub use edge::*;
pub use mcts::*;
pub use node::*;
pub use state_diff::*;
pub use task::*;
pub use util::*;

/// The identifier of an agent, essentially a u32.
#[derive(
    Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct AgentId(pub u32);
impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A{}", self.0)
    }
}
