// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "unsound_experiments")]
use clap::Parser;
#[derive(Debug, Parser)]
pub struct UnsoundExperimentArgs {
    /// Zero initilize variables.
    /// This is useful for experiments to see whether assigning constant values produces better
    /// performance by allowing CBMC to do more constant propegation.
    /// Unfortunatly, it is unsafe to use for production code, since it may unsoundly hide bugs.
    /// Marked as `unsound` to prevent use outside of experimental contexts.
    #[arg(long, hide_short_help = true, requires("enable_unstable"))]
    pub unsound_experiment_zero_init_vars: bool,
}

impl UnsoundExperimentArgs {
    pub fn process_args(&self) -> Vec<String> {
        self.print_warnings();
        let mut flags = vec![];
        if self.unsound_experiment_zero_init_vars {
            flags.push("--unsound-experiment-zero-init-vars".into());
        }
        flags
    }

    pub fn print_warnings(&self) {
        if self.unsound_experiment_zero_init_vars {
            eprintln!(
                "Warning: using --unsound-experiment-zero-init-vars can lead to unsound results"
            );
        }
    }
}
