/// Entrypoint module, it includes the CLI and its UI for display results
mod domain;
mod graph;
mod infra;
mod services;

use std::env;

use infra::DBRepository;
use services::*;

fn display(stats: &dto::Stadistics) {
    let mut output = String::new();
    output += format!("> AVG DAG DEPTH: {:.2}\n", stats.average_depth).as_str();
    output += format!("> AVG TXS PER DEPTH: {:.2}\n", stats.average_nodes_by_depth).as_str();
    output += format!("> AVG REF: {:.2}\n", stats.average_in_references).as_str();
    print!("{:}", output);
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
            let stats = stadistics::stats(&model_graph);
            display(&stats);
        }
        Err(e) => {
            let err_mesg = format!("The graph could not be loaded: {:?}", e);
            eprintln!("{:}", err_mesg);
        }
    }
}
