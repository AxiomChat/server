use std::{fs::File, io, path::PathBuf, str::FromStr, sync::Arc};

use zip::ZipArchive;

use crate::{logger, plugin::loader::PluginLoader, server::Server};

logger!(LOGGER "CLI");

pub fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();

    let mut in_dq = false;
    let mut in_q = false;
    let mut escape = false;

    for c in input.chars() {
        if escape {
            current.push(c);
            escape = false;
            continue;
        }

        match c {
            '\\' => {
                escape = true;
            }
            '"' if !in_q && !escape => {
                in_dq = !in_dq;
            }
            '\'' if !in_dq && !escape => {
                in_q = !in_q;
            }
            ' ' if !in_dq && !in_q => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if escape {
        current.push('\\');
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

pub fn require_args(args: &[String], required: &[&str]) -> bool {
    if required.len() + 1 > args.len() {
        LOGGER.error(format!("Required arguments: {}", required.join(" ")));
        return false;
    }

    true
}

pub fn start_cli(server: Arc<Server>, plugin_loader: PluginLoader) {
    std::thread::spawn(move || {
        loop {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            let args = parse_args(buf.trim());
            if args.len() == 0 {
                continue;
            }

            match args[0].as_str() {
                "install" => {
                    if require_args(&args, &["<path.vxp>"]) {
                        let path = PathBuf::from_str(&args[1]).unwrap();
                        if path.extension().and_then(|s| s.to_str()) == Some("vxp") {
                            LOGGER.info(format!("Installing {:?}", path));
                            let d = path.with_extension("");
                            if !d.exists() {
                                let file = File::open(&path).unwrap();
                                let mut archive = ZipArchive::new(file).unwrap();
                                archive.extract(&d).unwrap();
                            }
                        } else {
                            LOGGER.error("File must be .vxp");
                        }
                    }
                }
                "load" => {
                    if require_args(&args, &["<plugin-dir>"]) {
                        plugin_loader.load(&server.root.join("plugins").join(&args[1]));
                    }
                }
                _ => {
                    LOGGER.error(format!("Command not found ({})", args[0]));
                }
            }
        }
    });
}
