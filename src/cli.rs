use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    pub path: PathBuf,
    #[command(subcommand)]
    pub command: Action,
}

#[derive(Subcommand)]
pub enum Action {
    InitTodo {},
    AddPoint {
        #[arg(short)]
        description: String,
        #[arg(short)]
        checked: Option<bool>
    },
    ShowAllPoints {},
    ReinstallTable {},
    RemovePoint {
        #[arg(long)]
        id: i64
    },
    SetChecked {
        #[arg(short, action=ArgAction::SetTrue)]
        change_to: bool,
        #[arg(long)]
        id: i64
    },
    ChangeDescription {
        #[arg(long, value_name="to description")]
        to: String,
        #[arg(long)]
        id: i64
    }
}
