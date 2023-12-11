/// Entrypoint module, it includes the CLI and its UI for display results
mod domain;
mod graph;
mod infra;
mod services;

use std::env;

use infra::DBRepository;
use services::*;

fn display(stats: &dto::Statistics) {
    let mut output = String::new();
    output += format!("> AVG DAG DEPTH: {:.2}\n", stats.average_depth).as_str();
    output += format!("> AVG TXS PER DEPTH: {:.2}\n", stats.average_nodes_by_depth).as_str();
    output += format!("> AVG REF: {:.2}\n", stats.average_in_references).as_str();
    output += format!("> TRANS LAST: {:}\n", stats.last_transaction).as_str();
    output += format!(
        "> TRANS MOST IN REF: {:}\n",
        stats.most_referenced_transaction
    )
    .as_str();
    output += format_timestamps(&stats.range_timestamps).as_str();
    print!("{:}", output);
}
fn format_timestamps(timestamps: &Vec<(u32, u64)>) -> String {
    let mut output = String::new();
    output += "> TIMESTAMPS --> NUM TRANS \n";
    for (range, count) in timestamps.iter() {
        output += format!(
            "- {:}:{:} --> {:} trans\n",
            range,
            range + statistics::TIMESTAMP_RANGE,
            count
        )
        .as_str();
    }
    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_file = args.get(1);
    if path_file.is_none() {
        eprintln!("Command needs an argument");
        return ();
    }
    let repo = DBRepository::new(&path_file.unwrap());
    if repo.is_none() {
        eprintln!("The path file must be correct");
        return ();
    }

    match repo.unwrap().load() {
        Ok(model_graph) => {
            let stats = statistics::stats(&model_graph);
            display(&stats);
        }
        Err(e) => {
            let err_mesg = format!("The graph could not be loaded: {:?}", e);
            eprintln!("{:}", err_mesg);
        }
    }
}
