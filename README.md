# vacuum-cs

PostgreSQL vacuum tool using Rust by Code Solver. No need other PostgreSQL clients for execute this.

## Usage

```bash
vacuum-cs [OPTIONS]

Options:
  -H, --host <HOST>          Path to the PostgreSQL configuration file [default: localhost]
  -p, --port <PORT>          PostgreSQL port [default: 5432]
  -d, --database <DATABASE>  PostgreSQL SCAP database [default: scap]
  -u, --username <USERNAME>  SCAP DBA username [default: scap_admin]
  -P, --password <PASSWORD>  SCAP DBA password [default: scap_admin]
  -l, --logfile <LOGFILE>    Log file
  -v, --verbose              Verbose mode
  -h, --help                 Print help
  -V, --version              Print version
```

## Building

The container compile the binary for platforms Linux and Windows 64 bits.

### Linux

```bash
cargo build --release
```

### Windows

```bash
cargo build --release --target x86_64-pc-windows-gnu 
```