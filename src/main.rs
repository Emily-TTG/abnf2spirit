// SPDX-License-Identifier: X11
// Copyright (C) 2026 Emily "TTG" Banerjee <prs.ttg+abnf2spirit@pm.me>

use std::env;
use std::fs;
use std::process;

use std::io::Write;

use abnf::types::Node;
use abnf::types::Rule;
use abnf::types::TerminalValues;

use anyhow::Context;

fn fatal_read(
		path: &str,
		context: &'static str) -> Result<String, anyhow::Error> {

	fs::read_to_string(path).with_context(|| format!("{} '{}'", context, path))
}

fn sanitize_rule_name(name: &str) -> String {
	name.replace("-", "_")
}

fn intersperse_nodes(
		nodes: &Vec<Node>,
		out: &mut String,
		separator: &'static str) {

	let mut iterator = nodes.iter().peekable();

	while let Some(child) = iterator.next() {
		process_node(child, out);

		if iterator.peek().is_some() {
			out.push_str(separator);
		}
	}
}

fn process_node(node: &Node, out: &mut String) {
	if !out.ends_with([ ' ', '[', '(' ]) {
		out.push_str(" ");
	}

	match node {
		Node::Alternatives(children) => {
			intersperse_nodes(children, out, " | ");
		}
		Node::Concatenation(children) => {
			intersperse_nodes(children, out, " >> ");
		}
		Node::Repetition{ repeat, node } => {
			out.push_str("boost::spirit::x4::repeat(");

			if repeat.min().is_none() {
				out.push_str("0");
			}
			else {
				out.push_str(repeat.min().unwrap().to_string().as_str());
			}
			out.push_str(", ");

			if repeat.max().is_none() {
				out.push_str("boost::spirit::x4::repeat_inf");
			}
			else {
				out.push_str(repeat.max().unwrap().to_string().as_str());
			}
			out.push_str(")[");

			process_node(node, out);
			out.push_str("]");
		},
		Node::Rulename(name) => {
			match name.as_str() {
				"ALPHA" => out.push_str("boost::spirit::x4::alpha"),
				"DIGIT" => out.push_str("boost::spirit::x4::digit"),
				"DQUOTE" => out.push_str("boost::spirit::x4::lit('\"')"),
				"VCHAR" => out.push_str(
						"boost::spirit::x4::char_('\\x21', '\\x7e')"),
				"WSP" => out.push_str("boost::spirit::x4::char_(\" \\t\")"),
				"CRLF" => out.push_str("boost::spirit::x4::lit(\"\\r\\n\")"),
				_ => out.push_str(format!(
						"boost::spirit::x4::omit[{}]",
						sanitize_rule_name(name.as_str())).as_str())
			};
		},
		Node::Group(node) => {
			out.push_str("(");
			process_node(node, out);
			out.push_str(")");
		},
		Node::Optional(node) => {
			out.push_str("-(");
			process_node(node, out);
			out.push_str(")");
		},
		Node::String(string) => {
			out.push_str(format!(
					"boost::spirit::x4::no_case[boost::spirit::x4::lit(\"{}\")]",
					escape_string::escape(string.as_str())).as_str());
		},
		Node::TerminalValues(values) => {
			match values {
				TerminalValues::Range(min, max) => {
					out.push_str(format!(
							"boost::spirit::x4::char_('\\x{:x}'",
							min).as_str());

					if min != max {
						out.push_str(format!(", '\\x{:x}'", max).as_str());
					}

					out.push_str(")");
				},
				TerminalValues::Concatenation(values) => {
					if values.len() == 1 {
						out.push_str(format!(
								"boost::spirit::x4::char_('\\x{:x}')",
								values[0]).as_str());
					}
					else {
						eprintln!("skipping concatenated terminals");
					}
				}
			};
		},
		Node::Prose(prose) => {
			eprintln!("skipping prose '{}'", prose);
		}
	};
}

fn run() -> Result<(), anyhow::Error> {
	let argv: Vec<String> = env::args().collect();

	if argv.len() != 4 {
		eprintln!(
				"usage: grammar source.abnf template.inl.in generated.inl");

		process::exit(1);
	}

	let source: String = fatal_read(
			&argv[1],
			"unable to read source grammar")?;

	let template: String = fatal_read(
			&argv[2],
			"unable to read template")?;

	let rules: Vec<Rule> = abnf::rulelist(&source)?;

	let mut declarations = String::new();
	let mut definitions = String::new();
	let mut bindings = String::new();

	for rule in &rules {
		let sanitized: String = sanitize_rule_name(rule.name());

		declarations.push_str(format!(
				"\tinline boost::spirit::x4::rule<struct {}_id> {} = \"{}\";\n",
				sanitized,
				sanitized,
				rule.name()).as_str());

		definitions.push_str(
				format!("\tconst auto {}_def =", sanitized).as_str());

		process_node(rule.node(), &mut definitions);
		definitions.push_str(";\n");

		bindings.push_str(format!(
				"\tBOOST_SPIRIT_X4_DEFINE({})\n",
				sanitized).as_str());
	}

	let mut file = fs::File::create(&argv[3])?;

	let generated = format!(
			"{}\n{}\n{}",
			declarations,
			definitions,
			bindings);

	file.write_all(
			template.replace("{{GRAMMAR}}", generated.as_str()).as_bytes())?;

	Ok(())
}

fn main() {
	if let Err(error) = run() {
		eprintln!("{}", error);
		process::exit(1);
	}
}
