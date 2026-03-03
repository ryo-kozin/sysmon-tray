use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use sysinfo::{Disks, System, MINIMUM_CPU_UPDATE_INTERVAL};

// ── MCP JSON-RPC types ───────────────────────────────────────────────────────

#[derive(Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

// ── Metrics (numbers only — no PII) ─────────────────────────────────────────

#[derive(Serialize)]
struct SystemMetrics {
    cpu_usage_percent: f32,
    cpu_count_logical: usize,
    memory_total_bytes: u64,
    memory_used_bytes: u64,
    memory_usage_percent: f32,
    memory_available_bytes: u64,
    swap_total_bytes: u64,
    swap_used_bytes: u64,
    disk_total_bytes: u64,
    disk_free_bytes: u64,
    disk_usage_percent: f32,
    load_avg_1m: f64,
    load_avg_5m: f64,
    load_avg_15m: f64,
    uptime_secs: u64,
    process_count: usize,
}

fn get_process_count() -> usize {
    // Lightweight process count without loading full process list
    #[cfg(target_os = "linux")]
    {
        std::fs::read_dir("/proc")
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .filter(|e| {
                        e.file_name()
                            .to_str()
                            .map_or(false, |s| s.chars().all(|c| c.is_ascii_digit()))
                    })
                    .count()
            })
            .unwrap_or(0)
    }
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("sysctl")
            .args(["-n", "kern.proc.all"])
            .output()
            .ok()
            .and_then(|o| {
                // Each process entry is a fixed-size struct; count by dividing total size
                // Fallback: just count lines from ps
                if o.stdout.is_empty() {
                    Command::new("ps")
                        .args(["-e", "-o", "pid="])
                        .output()
                        .ok()
                        .map(|o| String::from_utf8_lossy(&o.stdout).lines().count())
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }
    #[cfg(target_os = "windows")]
    {
        // Lightweight fallback
        0
    }
}

fn collect_metrics(sys: &mut System, disks: &mut Disks) -> SystemMetrics {
    sys.refresh_cpu_all();
    sys.refresh_memory();
    disks.refresh(true);

    let memory_total = sys.total_memory();
    let memory_used = sys.used_memory();
    let memory_available = sys.available_memory();

    let (disk_total, disk_free) = disks
        .iter()
        .filter(|d| {
            let mp = d.mount_point();
            mp == std::path::Path::new("/") || mp == std::path::Path::new("C:\\")
        })
        .map(|d| (d.total_space(), d.available_space()))
        .next()
        .or_else(|| {
            disks
                .iter()
                .max_by_key(|d| d.total_space())
                .map(|d| (d.total_space(), d.available_space()))
        })
        .unwrap_or((0, 0));

    let load = System::load_average();

    SystemMetrics {
        cpu_usage_percent: sys.global_cpu_usage(),
        cpu_count_logical: sys.cpus().len(),
        memory_total_bytes: memory_total,
        memory_used_bytes: memory_used,
        memory_usage_percent: if memory_total > 0 {
            (memory_used as f32 / memory_total as f32) * 100.0
        } else {
            0.0
        },
        memory_available_bytes: memory_available,
        swap_total_bytes: sys.total_swap(),
        swap_used_bytes: sys.used_swap(),
        disk_total_bytes: disk_total,
        disk_free_bytes: disk_free,
        disk_usage_percent: if disk_total > 0 {
            ((disk_total - disk_free) as f32 / disk_total as f32) * 100.0
        } else {
            0.0
        },
        load_avg_1m: load.one,
        load_avg_5m: load.five,
        load_avg_15m: load.fifteen,
        uptime_secs: System::uptime(),
        process_count: get_process_count(),
    }
}

// ── MCP Protocol ─────────────────────────────────────────────────────────────

const SERVER_NAME: &str = "sysmon-mcp";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");
const PROTOCOL_VERSION: &str = "2024-11-05";

fn handle_initialize(_params: &Value) -> Value {
    json!({
        "protocolVersion": PROTOCOL_VERSION,
        "capabilities": {
            "tools": {}
        },
        "serverInfo": {
            "name": SERVER_NAME,
            "version": SERVER_VERSION
        }
    })
}

fn handle_tools_list() -> Value {
    json!({
        "tools": [
            {
                "name": "get_metrics",
                "description": "Get current system resource metrics. Returns only numerical values — no process names, file paths, usernames, or other personally identifiable information. Safe to share with AI assistants.",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            },
            {
                "name": "get_cpu",
                "description": "Get CPU usage percentage and logical core count.",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            },
            {
                "name": "get_memory",
                "description": "Get memory usage: total, used, available bytes and usage percentage.",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            },
            {
                "name": "get_disk",
                "description": "Get disk usage: total, free bytes and usage percentage.",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            },
            {
                "name": "get_load",
                "description": "Get system load averages (1m, 5m, 15m), uptime, and process count.",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            },
            {
                "name": "is_system_busy",
                "description": "Quick check: returns whether the system is under heavy load. Useful for AI assistants to decide whether to run resource-intensive tasks.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "cpu_threshold": {
                            "type": "number",
                            "description": "CPU usage percentage threshold (default: 80)",
                            "default": 80
                        },
                        "memory_threshold": {
                            "type": "number",
                            "description": "Memory usage percentage threshold (default: 85)",
                            "default": 85
                        }
                    },
                    "required": []
                }
            }
        ]
    })
}

fn handle_tools_call(params: &Value, sys: &mut System, disks: &mut Disks) -> Result<Value, String> {
    let tool_name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or("missing tool name")?;

    let metrics = collect_metrics(sys, disks);

    let content = match tool_name {
        "get_metrics" => serde_json::to_string_pretty(&metrics).unwrap(),

        "get_cpu" => {
            format!(
                "CPU Usage: {:.1}%\nLogical Cores: {}",
                metrics.cpu_usage_percent, metrics.cpu_count_logical
            )
        }

        "get_memory" => {
            format!(
                "Memory Usage: {:.1}%\nUsed: {:.1} GB / {:.1} GB\nAvailable: {:.1} GB\nSwap: {:.1} / {:.1} GB",
                metrics.memory_usage_percent,
                metrics.memory_used_bytes as f64 / 1_073_741_824.0,
                metrics.memory_total_bytes as f64 / 1_073_741_824.0,
                metrics.memory_available_bytes as f64 / 1_073_741_824.0,
                metrics.swap_used_bytes as f64 / 1_073_741_824.0,
                metrics.swap_total_bytes as f64 / 1_073_741_824.0,
            )
        }

        "get_disk" => {
            format!(
                "Disk Usage: {:.1}%\nFree: {:.1} GB / {:.1} GB",
                metrics.disk_usage_percent,
                metrics.disk_free_bytes as f64 / 1_073_741_824.0,
                metrics.disk_total_bytes as f64 / 1_073_741_824.0,
            )
        }

        "get_load" => {
            format!(
                "Load Average: {:.2} (1m) / {:.2} (5m) / {:.2} (15m)\nUptime: {} hours\nProcesses: {}",
                metrics.load_avg_1m,
                metrics.load_avg_5m,
                metrics.load_avg_15m,
                metrics.uptime_secs / 3600,
                metrics.process_count,
            )
        }

        "is_system_busy" => {
            let args = params.get("arguments").unwrap_or(&Value::Null);
            let cpu_thresh = args
                .get("cpu_threshold")
                .and_then(Value::as_f64)
                .unwrap_or(80.0) as f32;
            let mem_thresh = args
                .get("memory_threshold")
                .and_then(Value::as_f64)
                .unwrap_or(85.0) as f32;

            let cpu_busy = metrics.cpu_usage_percent > cpu_thresh;
            let mem_busy = metrics.memory_usage_percent > mem_thresh;

            format!(
                "busy: {}\ncpu_busy: {} ({:.1}% > {:.0}% threshold: {})\nmemory_busy: {} ({:.1}% > {:.0}% threshold: {})\nrecommendation: {}",
                cpu_busy || mem_busy,
                cpu_busy, metrics.cpu_usage_percent, cpu_thresh, cpu_busy,
                mem_busy, metrics.memory_usage_percent, mem_thresh, mem_busy,
                if cpu_busy && mem_busy {
                    "System is under heavy load. Defer resource-intensive tasks."
                } else if cpu_busy {
                    "CPU is busy. Consider waiting before running builds or heavy computation."
                } else if mem_busy {
                    "Memory is low. Avoid spawning many parallel processes."
                } else {
                    "System resources are available. Safe to proceed with tasks."
                }
            )
        }

        _ => return Err(format!("unknown tool: {tool_name}")),
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": content
        }]
    }))
}

// ── Main Loop ────────────────────────────────────────────────────────────────

fn send_response(stdout: &mut io::StdoutLock, resp: &JsonRpcResponse) {
    let json = serde_json::to_string(resp).unwrap();
    writeln!(stdout, "{json}").unwrap();
    stdout.flush().unwrap();
}

fn main() {
    // Minimal init: only CPU + memory — never load full process list
    let mut sys = System::new();
    sys.refresh_cpu_all();
    sys.refresh_memory();
    // First CPU reading is always 0; sleep for accurate baseline
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();

    let mut disks = Disks::new_with_refreshed_list();

    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();

    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };

        if line.trim().is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".into(),
                    id: Value::Null,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {e}"),
                    }),
                };
                send_response(&mut stdout, &resp);
                continue;
            }
        };

        if req.jsonrpc != "2.0" {
            continue;
        }

        // Notifications (no id) — just acknowledge
        if req.id.is_none() {
            // "notifications/initialized" etc — no response needed
            continue;
        }

        let id = req.id.unwrap_or(Value::Null);

        let result = match req.method.as_str() {
            "initialize" => Ok(handle_initialize(&req.params)),
            "tools/list" => Ok(handle_tools_list()),
            "tools/call" => {
                handle_tools_call(&req.params, &mut sys, &mut disks).map_err(|e| JsonRpcError {
                    code: -32602,
                    message: e,
                })
            }
            "ping" => Ok(json!({})),
            _ => Err(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", req.method),
            }),
        };

        let resp = match result {
            Ok(r) => JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id,
                result: Some(r),
                error: None,
            },
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id,
                result: None,
                error: Some(e),
            },
        };

        send_response(&mut stdout, &resp);
    }
}
