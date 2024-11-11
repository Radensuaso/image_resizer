# Tool Kit

## Install

```bash
cargo build --release

cargo install --path .

export OPENWEATHER_API_KEY="your_api_key_here"
```

```powershell
cargo build --release

cargo install --path .

[System.Environment]::SetEnvironmentVariable("OPENWEATHER_API_KEY", "your_api_key_here", "User")
```

## How to use

```bash
tool_kit resize_image <mb> <input_image> <output_image>

tool_kit resize_bulk_images <mb> <input_folder> <output_folder>

tool_kit web_crawler <URL> <max_depth>

tool_kit weather <city>
```
