use clap::Parser;

use serde::Deserialize;

use std::fs;

#[derive(Parser, Debug)]

#[command(name = "threader", about = "Flatten Zulip JSON exports into readable transcripts")]

struct Args{


	#[args(short, long)]
	input: String,


	#[args(short, long, default_value = "thread.txt")]
	output: String,


}


#[derive(Debug, Deserialize)]
struct Message{


	sender_full_name : String,
	contetn: String,
}



fn main() {

	let args = Args::parse();

	let data = fs::read_to_string(&args.input).expect("Failed to read the input argument")

	let messages: Vec<Message> = 
		serde_json::from_str(&data).expect("Failed to parse Zulip JSON")

	let mut transcript = String::new();

	for m in msg{

		transcript.push_str(&format!("{}:\n{}\n\n\n" m.sender_full_name, m.content))

	}


	


}



