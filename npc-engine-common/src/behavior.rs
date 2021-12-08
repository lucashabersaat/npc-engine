use std::fmt;

use crate::{AgentId, Domain, StateRef, Task};

/// A possibly-recursive set of possible tasks
pub trait Behavior<D: Domain>: fmt::Display + 'static {
    /// Returns dependent behaviors.
    fn get_dependent_behaviors(&self) -> &'static [&'static dyn Behavior<D>] {
        &[]
    }

    /// Collects valid tasks for the given agent in the given world state.
    #[allow(unused)]
    fn add_own_tasks(&self, state: StateRef<D>, agent: AgentId, tasks: &mut Vec<Box<dyn Task<D>>>) {}

    /// Returns if the behavior is valid for the given agent in the given world state.
    fn is_valid(&self, state: StateRef<D>, agent: AgentId) -> bool;

    /// Helper method to recursively collect all valid tasks for the given agent in the given world state.
    fn add_tasks(&self, state: StateRef<D>, agent: AgentId, tasks: &mut Vec<Box<dyn Task<D>>>) {
        self.add_own_tasks(state, agent, tasks);
        self.get_dependent_behaviors()
            .iter()
            .filter(|behavior| behavior.is_valid(state, agent))
            .for_each(|behavior| behavior.add_tasks(state, agent, tasks));
    }
}