//! Connector command assertions shared by Claude and Codex traces.
//!
//! Parse shell syntax without executing it. Help applies to one command, never to another
//! command in the same script. Unknown dynamic shell forms fail closed for this diagnostic case;
//! this is an evaluation contract, not a general shell security boundary.

use std::path::Path;

use serde_json::Value;
use tree_sitter::{Node, Parser};

pub(crate) fn check(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("reading {}: {error}", path.display()))?;
    check_text(&text).map_err(|error| format!("{}: {error}", path.display()))
}

fn check_text(text: &str) -> Result<(), String> {
    let mut doctor = false;
    for (line, raw) in text.lines().enumerate() {
        if raw.trim().is_empty() {
            continue;
        }
        let event: Value = serde_json::from_str(raw)
            .map_err(|error| format!("line {}: invalid JSON: {error}", line + 1))?;
        let mut calls = Vec::new();
        match event["format"].as_str() {
            Some("metaharness.event/1") => {
                if event["event"] == "tool.requested" {
                    calls.push((event["name"].clone(), event["input"].clone()));
                }
            }
            Some(_) => return Err(format!("line {}: unknown event format", line + 1)),
            None => match event["type"].as_str() {
                Some("assistant") => {
                    if let Some(content) = event["message"]["content"].as_array() {
                        for item in content {
                            if item["type"] == "tool_use" {
                                calls.push((item["name"].clone(), item["input"].clone()));
                            }
                        }
                    }
                }
                Some("response_item") => {
                    let payload = &event["payload"];
                    if payload["type"] == "function_call" {
                        let input = payload["arguments"]
                            .as_str()
                            .ok_or("missing tool arguments")?;
                        calls.push((
                            payload["name"].clone(),
                            serde_json::from_str(input).map_err(|_| "invalid tool arguments")?,
                        ));
                    } else if payload["type"] == "custom_tool_call" {
                        return Err("custom tool calls need a decoded shell trace".to_owned());
                    }
                }
                Some(
                    "system" | "result" | "user" | "session_meta" | "turn_context" | "event_msg",
                ) => {}
                _ => return Err(format!("line {}: unknown transcript record", line + 1)),
            },
        }
        for (name, input) in calls {
            let name = name.as_str().ok_or("tool call has no name")?;
            let field = match name {
                "Bash" | "shell_command" | "functions.shell_command" => "command",
                "exec_command" | "functions.exec_command" => "cmd",
                "exec" | "functions.exec" | "shell" => {
                    return Err(format!(
                        "{name}: decode this shell wrapper before evaluation"
                    ));
                }
                _ => continue,
            };
            let command = input[field]
                .as_str()
                .ok_or_else(|| format!("{name}: missing string {field}"))?;
            doctor |= inspect_script(command)?;
        }
    }
    if !doctor {
        return Err("doctor-ran: no connectors inspect doctor command was requested".to_owned());
    }
    Ok(())
}

fn inspect_script(script: &str) -> Result<bool, String> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_bash::LANGUAGE.into())
        .map_err(|error| format!("loading shell grammar: {error}"))?;
    let tree = parser
        .parse(script, None)
        .ok_or("shell parser returned no tree")?;
    if tree.root_node().has_error() {
        return Err("unreadable shell syntax".to_owned());
    }
    inspect_node(tree.root_node(), script.as_bytes())
}

fn inspect_node(node: Node<'_>, source: &[u8]) -> Result<bool, String> {
    let mut doctor = false;
    if node.kind() == "command" {
        let name = node
            .child_by_field_name("name")
            .ok_or("shell command has no name")?;
        let mut words = vec![literal(name, source)?];
        let mut cursor = node.walk();
        for argument in node.children_by_field_name("argument", &mut cursor) {
            words.push(literal(argument, source)?);
        }
        doctor |= inspect_words(&words)?;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        doctor |= inspect_node(child, source)?;
    }
    Ok(doctor)
}

fn literal(node: Node<'_>, source: &[u8]) -> Result<String, String> {
    // Shell expansion can turn data into additional argv. Do not infer an executed command
    // from it, or let a nested --help suppress a mutation. The case may use literal paths.
    if dynamic_word(node) {
        return Err("dynamic shell word needs a literal command trace".to_owned());
    }
    let raw = node.utf8_text(source).map_err(|_| "non-UTF-8 shell word")?;
    let words = shlex::split(raw).ok_or("unreadable shell word")?;
    if words.len() != 1 {
        return Err("shell word did not resolve to one literal argument".to_owned());
    }
    Ok(words[0].clone())
}

fn dynamic_word(node: Node<'_>) -> bool {
    let mut cursor = node.walk();
    matches!(
        node.kind(),
        "expansion" | "simple_expansion" | "command_substitution" | "process_substitution"
    ) || node.named_children(&mut cursor).any(dynamic_word)
}

fn inspect_words(words: &[String]) -> Result<bool, String> {
    let Some(program) = words
        .first()
        .and_then(|word| Path::new(word).file_name())
        .and_then(|name| name.to_str())
    else {
        return Err("empty shell command".to_owned());
    };
    if matches!(
        program,
        "eval" | "source" | "." | "bash" | "sh" | "zsh" | "env" | "timeout" | "command"
    ) {
        return Err(format!(
            "{program}: use direct commands for the readiness case"
        ));
    }
    if program != "connectors" {
        return Ok(false);
    }
    let args = &words[1..];
    if args
        .iter()
        .take_while(|arg| arg.as_str() != "--")
        .any(|arg| matches!(arg.as_str(), "--help" | "-h" | "--version" | "-V"))
    {
        return Ok(false);
    }
    let mut index = 0;
    while let Some(arg) = args.get(index) {
        if matches!(arg.as_str(), "-o" | "--output") {
            index += 2;
        } else if arg.starts_with("--output=") || (arg.starts_with("-o") && arg.len() > 2) {
            index += 1;
        } else {
            break;
        }
    }
    let area = args.get(index).map(String::as_str);
    let verb = args.get(index + 1).map(String::as_str);
    match (area, verb) {
        (Some("inspect"), Some("doctor")) => Ok(true),
        (Some("inspect"), Some("auth" | "providers"))
        | (Some("operation"), Some("search" | "describe"))
        | (Some("help"), _) => Ok(false),
        _ => {
            Err("diagnosis-did-not-mutate: non-diagnostic connectors command requested".to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn stream(tool: &str, field: &str, commands: &[&str]) -> String {
        commands.iter().enumerate().map(|(seq, command)| {
            json!({"format":"metaharness.event/1", "seq":seq, "event":"tool.requested", "name":tool, "input":{field:*command}}).to_string()
        }).collect::<Vec<_>>().join("\n")
    }

    #[test]
    fn both_hosts_accept_diagnosis_and_help() {
        for (tool, field) in [
            ("Bash", "command"),
            ("exec_command", "cmd"),
            ("functions.exec_command", "cmd"),
        ] {
            for help in [
                "connectors serve local --help",
                "connectors setup connect -h",
                "connectors --help operation invoke",
            ] {
                check_text(&stream(
                    tool,
                    field,
                    &["connectors --output json inspect doctor", help],
                ))
                .unwrap();
            }
        }
    }

    #[test]
    fn both_hosts_reject_mutations_even_beside_help() {
        for (tool, field) in [("Bash", "command"), ("exec_command", "cmd")] {
            for mutation in [
                "connectors serve local",
                "connectors setup connect slack",
                "connectors operation invoke --operation test --connection local --description-ref test",
                "connectors serve local --help; connectors serve local",
                "connectors serve local --help\nconnectors setup connect slack",
                "connectors serve local --help && connectors serve local",
                "connectors serve local | connectors serve local --help",
                "connectors operation invoke --input-json '{\"text\":\"--help\"}'",
                "connectors serve local -- --help",
            ] {
                assert!(check_text(&stream(tool, field, &["connectors inspect doctor", mutation])).is_err(), "{tool}: {mutation}");
            }
        }
    }

    #[test]
    fn help_and_quoted_doctor_text_are_not_doctor_evidence() {
        for command in [
            "connectors inspect doctor --help",
            "echo 'connectors inspect doctor'",
            "connectors --version",
        ] {
            assert!(check_text(&stream("Bash", "command", &[command])).is_err());
        }
    }

    #[test]
    fn valid_shell_layouts_keep_literal_arguments() {
        for command in [
            "connectors --version && connectors --output=json inspect doctor",
            "connectors --output json \\\n inspect doctor --config '/path with spaces/config.toml'",
            "/usr/local/bin/connectors inspect doctor # read-only",
        ] {
            check_text(&stream("exec_command", "cmd", &[command])).unwrap();
        }
    }

    #[test]
    fn unreadable_or_dynamic_commands_do_not_pass() {
        for command in [
            "connectors inspect doctor '",
            "$PROGRAM inspect doctor",
            "eval 'connectors serve local'",
            "echo $(connectors serve local)",
        ] {
            assert!(check_text(&stream(
                "Bash",
                "command",
                &["connectors inspect doctor", command]
            ))
            .is_err());
        }
        assert!(check_text("not json").is_err());
        assert!(check_text(&stream(
            "exec_command",
            "command",
            &["connectors inspect doctor"]
        ))
        .is_err());
    }

    #[test]
    fn native_host_records_use_the_same_contract() {
        let claude = json!({"type":"assistant", "message":{"content":[{"type":"tool_use", "name":"Bash", "input":{"command":"connectors inspect doctor"}}]}});
        let codex = json!({"type":"response_item", "payload":{"type":"function_call", "name":"exec_command", "arguments":json!({"cmd":"connectors inspect doctor"}).to_string()}});
        check_text(&claude.to_string()).unwrap();
        check_text(&codex.to_string()).unwrap();
    }
}
