# Windows 11 LTSC Activator

A Rust-based activator for Windows 11 Enterprise LTSC 2024.

## Features

- Multi-workspace Rust architecture
- Embedded license files (single executable)
- Command-line interface with customization options
- Dry-run mode for testing
- Verbose logging support

## Building

```bash
cargo build --release
```

The executable will be located at `target/release/win11-activator.exe`

## Usage

**Run as Administrator**

### Basic Usage

```bash
win11-activator.exe
```

### Custom KMS Server

```bash
win11-activator.exe --kms-server kms.example.com
```

### Custom Product Key

```bash
win11-activator.exe --product-key YOUR-KEY-HERE
```

### Dry Run (Test Mode)

```bash
win11-activator.exe --dry-run --verbose
```

### All Options

```bash
win11-activator.exe --help
```

## Command Line Options

- `-s, --kms-server <SERVER>` - KMS server address (default: kms.digiboy.ir)
- `-k, --product-key <KEY>` - Windows product key (default: M7XTQ-FN8P6-TTKYV-9D4CC-J462D)
- `-d, --dry-run` - Preview actions without executing
- `-v, --verbose` - Show detailed output

## Alternative KMS Servers

If the default KMS server doesn't work, try these alternatives:

- 54.223.212.31
- kms.cnlic.com
- kms.chinancce.com
- kms.ddns.net
- franklv.ddns.net
- k.zpale.com
- m.zpale.com
- mvg.zpale.com
- kms.shuax.com
- kensol263.imwork.net:1688
- kms.loli.best
- kms.vudy.net

## Architecture

```
windows-11-ltsc-activator/
├── win11-act-types/      # Shared types and error definitions
├── win11-act-embed/      # Embedded license files
├── win11-act-core/       # Core activation logic
└── win11-act-cli/        # CLI application
```

## License

MIT
