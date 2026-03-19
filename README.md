# sketchytop

A minimal sketchybar plugin written in rust that shows basic performance stats.

Designed for [SketchyBar](https://github.com/FelixKratz/SketchyBar).

## Features
- **High Performance**: Polls system stats with near-zero overhead.
- **Dynamic Colors**: Labels change color (Green/Orange/Red) based on resource usage.
- **Configurable**: Easily adjust update intervals and item names.
- **Minimalistic**: Focused on delivering high-signal information (CPU % and RAM GB).

## Installation

### Via Cargo
```bash
cargo install sketchytop
```

### From Source
```bash
git clone https://github.com/mackrus/sketchytop.git
cd sketchytop
cargo build --release
```

The binary will be located at `target/release/sketchytop`.

## Usage
Add the following to your `sketchybarrc`:

```bash
# Start the helper
/path/to/sketchytop --interval 1000 &
```

### Options
```bash
-i, --interval <INTERVAL>  Update interval in milliseconds [default: 1000]
    --ram-item <RAM_ITEM>  RAM item name in SketchyBar [default: ram]
    --cpu-item <CPU_ITEM>  CPU item name in SketchyBar [default: cpu.percent]
-h, --help                 Print help
-V, --version              Print version
```

## License
MIT
