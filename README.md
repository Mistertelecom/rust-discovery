# Network Sniffer

A Rust-based network packet sniffer using Npcap.

## Prerequisites

1. Install Npcap SDK:
   - Download from: https://npcap.com/#download
   - Run the installer and follow the instructions

2. Set NPCAP_SDK_PATH environment variable:
   - Windows:
     ```cmd
     setx NPCAP_SDK_PATH "C:\Program Files\Npcap SDK"
     ```
   - Linux/Mac:
     ```bash
     export NPCAP_SDK_PATH="/path/to/npcap/sdk"
     ```

3. Restart your terminal/IDE

## Building the Project

```bash
cargo build
```

## Running the Sniffer

```bash
cargo run
```

## Troubleshooting

If you get errors about missing NPCAP_SDK_PATH:
- Verify the environment variable is set correctly
- Restart your terminal/IDE
- Ensure the Npcap SDK is installed in the specified location
