extern crate serde;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use structopt::StructOpt;
use serde::Serialize;

#[derive(StructOpt)]
struct Options {
	#[structopt(short="o", default_value="inventory.csv")]
	output: String,
	#[structopt(short="f", long, default_value=".")]
	folder: String,
}

#[derive(Debug, Serialize)]
struct FolderSummary {
	path: String,
	is_go: bool,
	is_node: bool,
	is_java: bool,
	is_rust: bool,
	is_code_project: bool,
}

impl FolderSummary {
	fn new() -> FolderSummary {
		FolderSummary{
			is_go: false,
			is_node: false,
			is_java: false,
			is_rust: false,
			is_code_project: false,
			path: "".to_string()
		}
	}

	fn accept<'a>(&mut self, file: String, full_path: &'a str) {
		if file.eq_ignore_ascii_case("package.json") {
			self.is_node = true;
			self.is_code_project = true;
		}
		if file.eq_ignore_ascii_case("cargo.toml") {
			self.is_rust = true;
			self.is_code_project = true;
		}
		if file.eq_ignore_ascii_case("go.mod") {
			self.is_go = true;
			self.is_code_project = true;
		}
		if file.eq_ignore_ascii_case("pom.xml") {
			self.is_go = true;
			self.is_code_project = true;
		}
		if file.eq_ignore_ascii_case("build.gradle") {
			self.is_go = true;
			self.is_code_project = true;
		}
		if file.contains(".go") {
			self.is_go = true;
			self.is_code_project = true;
		}
		if self.is_code_project {
			self.path = full_path.to_string();
		}
	}
}

fn list(original_path: String, projects_list: &mut Vec<FolderSummary>) {
	let paths = fs::read_dir(&original_path);
	match paths {
		Ok(p) => {
			let mut summary = FolderSummary::new();
			for path in p {
				let dir_entry = &path.unwrap();
				if let Ok(metadata) = dir_entry.metadata() {
					if metadata.is_dir() {
						let fpath = format!("{}\\{}", original_path, dir_entry.file_name().into_string().unwrap());
						list(fpath, projects_list);
					} else {
						summary.accept(dir_entry.file_name().into_string().unwrap(), &original_path);
					}
				}
			}
			if summary.is_code_project {
				projects_list.push(summary);
			}
		},
		Err(e) => {
			println!("Error for path {}: {}", original_path, e);
		}
	}
}

fn main() {
	let opt = Options::from_args();
	let mut projects_list: Vec<FolderSummary> = Vec::new();
	let file_path = Path::new(&opt.output);
	let file = File::create(&file_path);
	match file {
		Ok(f) => {
			let mut wtr = csv::Writer::from_writer(f);
			println!("Results available at: {}", opt.output);
			let start_time = Instant::now();
			list(opt.folder, &mut projects_list);
			let total = projects_list.len();
			for i in 0..projects_list.len() {
				let p = projects_list.get(i).unwrap();
				wtr.serialize(p);
			}
			wtr.flush().unwrap();
			println!("Finished in {:?}. Found: {} projects", start_time.elapsed(), total);
		},
		Err(e) => {
			println!("Cannot create output file {}: {}", opt.output, e);
		}
	}
}