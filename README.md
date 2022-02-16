# Simple Status

![simple_status2](https://user-images.githubusercontent.com/36408549/154275556-006f7559-f567-4326-9b24-145f68cc009c.png)

Easy, Simple, Clean. Making status bars reliable and up-to-date. This project was inspired by [dwmblocks](https://github.com/torrinfail/dwmblocks) but is a Rust alternative with a toml configuration file.

## Installation

Compiling `simple_status` yourself doesn't require much.

### Installation Dependencies

All dependencies are likely readily available in your repository.

- **Cargo**
- **libX11**
- **git**

### Installation Process (Compile)

1. Run `./install.sh`
2. *Optional* - Move default config to the simple_status config directory: `mkdir -p ~/.config/simple_status && cp config.toml ~/.config/simple_status`

## Config

The default configuration can be found at `~/config.toml`. The config file must satisfy a `seperator`. The `seperator` is the string injected between the module outputs, you could think of it as the spacer, or hence the name, a seperator.

### Modules

Modules can either define a command to be executed and you use the output as return value from the module or configure a built-in module. If no `built_in` or `command` the module will not display. The order in which modules are defined down the config file will map left to right on the bar. For example the first module defined will become the left-most module on the status bar and the last will become the righ-most.

#### Optional Module Config

- `update` Should the module update.
- `delay` The delay in milliseconds between updates.

### Built-in Modules

- `cpu` CPU usage as a percentage.
- `mem` Memory usage as a percentage.
- `uptime` Uptime in the format `hours:mins:seconds`
- `date` Date in the format `day month year`
- `time` Time in `hours:mins:seconds`
- `load` Load as an average of the past minute
- `load_all` Load average in the format `one five fifteen`

``` toml
seperator = "|"

[[module]]
prefix = "Time: "
built_in = "time"
```

### Command Modules

To create a command module all that needs to be defined is a module with a command. A `built_in` should be defined, however the result would likely not result in an error and would be ignored.

``` toml
seperator = "|"

[[module]]
prefix = "Kernel: "
command = "uname -r"
```

## Under the hood

All simple_status is doing effectivley is `xsetroot -name <output>` with the output being the infomation generated from the config file.

### xlib

In `~/src/status.rs` simple_status has to do some calling of the X11 C API xlib. Due to Rust not being able to know if this could could be safe or unsafe all calls to see functions must be wrapped with an `unsafe` scope. For the most part this should be completely safe. X11 has been out for a long time now (you could see this as a negative or a positive) and any issues have likely been ironed out.

Using xlib does how ever mean at compile-time it must be linked to the X11 library, this can be seen in `~/build.rs` and is why `libX11` is required.
