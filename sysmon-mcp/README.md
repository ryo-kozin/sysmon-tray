# sysmon-mcp

A privacy-first [Model Context Protocol](https://modelcontextprotocol.io/) (MCP) server that exposes system metrics to AI coding assistants like Claude Code and Cursor.

**Only numerical values are shared** — no process names, file paths, usernames, or any personally identifiable information.

## Why?

AI coding assistants are blind to your system state. They don't know if your machine is thrashing on memory or if disk space is critically low. `sysmon-mcp` bridges that gap — giving your AI assistant just enough system awareness to make better decisions, without compromising your privacy.

**Example conversations you can have:**

- "Is my system too busy to run a heavy build right now?"
- "How much disk space do I have left?"
- "Check if my memory is running low before starting Docker"
- "Give me a system health overview"

## Install

```bash
cd sysmon-mcp
cargo install --path .
```

Or build manually:

```bash
cargo build --release
# Binary: target/release/sysmon-mcp
```

## Configure

### Claude Code

Add to `~/.claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "sysmon": {
      "command": "sysmon-mcp"
    }
  }
}
```

### Cursor

Add to your Cursor MCP settings:

```json
{
  "mcpServers": {
    "sysmon": {
      "command": "sysmon-mcp"
    }
  }
}
```

## Available Tools

| Tool             | Description                                                           |
| ---------------- | --------------------------------------------------------------------- |
| `get_metrics`    | Full system snapshot (CPU, memory, disk, load, uptime, process count) |
| `get_cpu`        | CPU usage % and logical core count                                    |
| `get_memory`     | Memory total/used/available, usage %, swap                            |
| `get_disk`       | Disk total/free and usage %                                           |
| `get_load`       | Load averages (1/5/15m), uptime, process count                        |
| `is_system_busy` | Quick check with configurable CPU/memory thresholds                   |

### Example: `get_metrics` output

```json
{
  "cpu_usage_percent": 23.4,
  "cpu_count_logical": 10,
  "memory_total_bytes": 17179869184,
  "memory_used_bytes": 12073740288,
  "memory_usage_percent": 70.3,
  "memory_available_bytes": 5106128896,
  "swap_total_bytes": 4294967296,
  "swap_used_bytes": 1073741824,
  "disk_total_bytes": 494384795648,
  "disk_free_bytes": 123596198912,
  "disk_usage_percent": 75.0,
  "load_avg_1m": 2.45,
  "load_avg_5m": 3.12,
  "load_avg_15m": 2.89,
  "uptime_secs": 432000,
  "process_count": 384
}
```

### Example: `is_system_busy`

```
busy: false
cpu_busy: false (23.4% > 80% threshold: false)
memory_busy: false (70.3% > 85% threshold: false)
recommendation: System resources are available. Safe to proceed with tasks.
```

Custom thresholds:

```json
{ "cpu_threshold": 50, "memory_threshold": 60 }
```

## Privacy

This server is designed with privacy as a core principle:

- All metric fields are strictly numeric
- No process names, command lines, or arguments
- No file paths or mount point names
- No usernames or hostnames
- No network interface names or addresses
- Safe to use with any AI assistant

Your process list, file system layout, and personal data never leave your machine — the AI only sees aggregate numbers like "CPU: 23.4%, Memory: 70.3%".

## Performance

| Metric     | sysmon-mcp | Activity Monitor |
| ---------- | ---------- | ---------------- |
| Memory     | ~9 MB      | ~89 MB           |
| Binary     | 414 KB     | —                |
| CPU (idle) | ~0%        | ~1-3%            |

## Troubleshooting

**"Command not found" after `cargo install`**
Ensure `~/.cargo/bin` is in your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**AI assistant can't connect**

1. Verify the binary is installed: `which sysmon-mcp`
2. Test manually: `echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1.0"}}}' | sysmon-mcp`
3. Check your config file path matches your AI assistant's expected location

**Metrics show 0% CPU on first call**
This is normal — the first reading after startup may show 0% as the CPU usage calculation requires two data points. Subsequent calls will show accurate values.

## Protocol

MCP over JSON-RPC 2.0 via stdio. Compatible with MCP protocol version `2024-11-05`.

## License

[MIT](../LICENSE)
