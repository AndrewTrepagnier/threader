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






