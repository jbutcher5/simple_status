# Simple Status

Easy, Simple, Clean. Making status bars reliable and up-to-date.

## Installation

Compiling `simple_status` yourself doesn't require much.

### Installation Dependencies

All dependencies are likely readily available in your repository.

- **Cargo**
- **libX11**
- **git**

### Installation Process (Compile)

1. Run `./install.sh`
2. *Optional* - Move default config to the simple_status config directory: `mkdir -p ~/.config/simple_status && cp config.yaml ~/.config/simple_status`

## Config

The default configuration can be found at `~/config.toml`.

### Modules

Modules can either define a command to be executed and you the output as return value from the module or configure a built-in module.

Each module defined (even built-in modules must be defined) must satisfy a `prefix` and `modules` and `seperator` .

``` toml
modules = ["time"]
seperator = "|"

[module."time"]
prefix = "Time: "
```

### Built-in Modules

- `cpu` CPU usage as a percentage.
- `mem` Memory usage as a percentage.
- `uptime` Uptime in the format `hours:mins:seconds`
- `date` Date in the format `day month year`
- `time` Time in `hours:mins:seconds`
- `load` Load as an average of the past minute
- `load_all` Load average in the format `one five fifteen`

### Command Modules

To create a command module all that needs to be defined is a module with a command.

``` toml
modules = ["kernel"]

[modue."kernel"]
prefix = "Kernel: "
command = "uname -r"
```
