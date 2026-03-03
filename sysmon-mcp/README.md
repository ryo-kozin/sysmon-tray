# sysmon-mcp

A privacy-first [Model Context Protocol](https://modelcontextprotocol.io/) (MCP) server that exposes system metrics to AI coding assistants like Claude Code and Cursor.

**Only numerical values are shared** — no process names, file paths, usernames, or any personally identifiable information.

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

## Performance

| Metric     | sysmon-mcp | Activity Monitor |
| ---------- | ---------- | ---------------- |
| Memory     | ~9 MB      | ~89 MB           |
| Binary     | 414 KB     | —                |
| CPU (idle) | ~0%        | ~1-3%            |

## Protocol

MCP over JSON-RPC 2.0 via stdio. Compatible with MCP protocol version `2024-11-05`.

## License

[MIT](../LICENSE)
