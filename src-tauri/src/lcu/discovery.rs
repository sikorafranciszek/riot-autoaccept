use std::fs;
use std::path::PathBuf;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tracing::debug;

#[derive(Debug, Clone)]
pub struct LcuCredentials {
    pub port: u16,
    pub token: String,
    pub source: &'static str,
}

const PROCESS_NAMES: &[&str] = &["LeagueClientUx.exe", "LeagueClientUx"];

/// Try to detect a running LeagueClientUx process and parse its command-line
/// arguments for `--app-port` and `--remoting-auth-token`. Falls back to the
/// lockfile written by the client.
pub fn detect_lcu() -> Option<LcuCredentials> {
    if let Some(creds) = from_process() {
        return Some(creds);
    }
    from_lockfile()
}

fn from_process() -> Option<LcuCredentials> {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    for (_pid, process) in sys.processes() {
        let name = process.name().to_string_lossy();
        if !PROCESS_NAMES.iter().any(|target| name.eq_ignore_ascii_case(target)) {
            continue;
        }

        let cmd: Vec<String> = process
            .cmd()
            .iter()
            .map(|s| s.to_string_lossy().into_owned())
            .collect();

        let port = find_arg_value(&cmd, "--app-port=").and_then(|v| v.parse::<u16>().ok());
        let token = find_arg_value(&cmd, "--remoting-auth-token=");

        if let (Some(port), Some(token)) = (port, token) {
            debug!(port, "found LCU credentials from process");
            return Some(LcuCredentials {
                port,
                token,
                source: "process",
            });
        }
    }

    None
}

fn find_arg_value(args: &[String], prefix: &str) -> Option<String> {
    args.iter()
        .find_map(|arg| arg.strip_prefix(prefix).map(|s| s.to_owned()))
}

fn lockfile_paths() -> Vec<PathBuf> {
    let mut paths = vec![
        PathBuf::from("C:\\Riot Games\\League of Legends\\lockfile"),
        PathBuf::from("C:\\Program Files\\Riot Games\\League of Legends\\lockfile"),
        PathBuf::from("C:\\Program Files (x86)\\Riot Games\\League of Legends\\lockfile"),
        PathBuf::from("/Applications/League of Legends.app/Contents/LoL/lockfile"),
    ];

    if let Some(home) = dirs::home_dir() {
        paths.push(home.join("Riot Games").join("League of Legends").join("lockfile"));
    }

    paths
}

fn from_lockfile() -> Option<LcuCredentials> {
    for path in lockfile_paths() {
        if !path.exists() {
            continue;
        }
        let Ok(contents) = fs::read_to_string(&path) else {
            continue;
        };
        let parts: Vec<&str> = contents.trim().split(':').collect();
        if parts.len() >= 4 {
            let port = parts[2].parse::<u16>().ok()?;
            let token = parts[3].to_owned();
            debug!(path = %path.display(), port, "found LCU credentials from lockfile");
            return Some(LcuCredentials {
                port,
                token,
                source: "lockfile",
            });
        }
    }
    None
}
