use clap::Parser;
use hyprland::data::*;
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use hyprland::prelude::*;
use hyprland::shared::HyprError;
use hyprland::Result;

/// Hyprland helper to open the next empty workspace
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Maximum number of available workspaces.
    #[arg(short, long = "max", default_value_t = 10)]
    max_workspaces: i32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let workspaces = Workspaces::get()?.to_vec();
    let workspace_ids = workspaces
        .iter()
        // Only count workspaces with open windows
        .filter(|w| w.windows > 0)
        .map(|w| w.id)
        .collect::<Vec<i32>>();

    for id in 1..args.max_workspaces + 1 {
        if workspace_ids.contains(&id) {
            continue;
        }

        return Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
            id,
        )));
    }

    Err(HyprError::NotOkDispatch(String::from(
        "Every available workspace is already open",
    )))
}
