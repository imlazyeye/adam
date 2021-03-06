# adam

<p align="center">
  <img width="400" height="400" src=assets/adam-200x200.png>
</p>

> I am thy creature: I ought to be thy Adam, but I am rather the fallen angel, whom thou drivest from joy for no misdeed.
> -- Frankenstein's Monster in *Frankenstein* by Mary Shelly

adam is a command-line utility for compiling Gms2 projects on Windows 10 and Mac OS Catalina. Invoking adam is trivial:

```sh
adam run
```

This will compile your project, run it, and give you stdout (`"show_debug_message"`) with colorization and links. `adam` supports compiling with the VM (default) and the YYC (by passing in `--yyc`). `adam` also supports faster recompilation than Gms2 does, so if users recompile a game without making changes, their game will instantly load, without invoking the compiler at all. This is especially useful, since `adam` easily allows you to run multiple instances of your game at the same time on your machine.

`adam` will place all its generated artifacts within a folder relative to the working directory -- by default, it will use `"target"` as its output output. **It is highly advised that you add your output directory to your .gitignore.**

Additionally, `adam` has the commands `build`, which builds a project without running it (but will report compile errors), and `clean`, which cleans the output directory.

## LIMITATIONS

To use `adam`, you must be using Gms2.3 in the non-beta (stable) channel, and your runtime must be exactly `2.3.0.401`. In the future, these limitations will be eased going *forward* (we will never support earlier than `2.3.0.401`, however).

Additionally, if your install path for the Gms2 executable is not in the standard location, see the `--gms2-exe-path` flag below.

## INSTALLATION

[Windows 10 and macOS Catalina binaries are always available in our releases page.](https://github.com/NPC-Studio/adam/releases) If you do not use a package manager, and are not a Rust programmer, this is the best way to get `adam`. After downloading your binary, you will probably want to add the package to your PATH so you can easily use it throughout your projects.

If you're a **macOS Homebrew** user, then you can install `adam` through homebrew:

```sh
brew tap sanbox-irl/homebrew-adam
brew install adam
```

If you're a Windows **scoop** user, then you can install `adam` through scoop:

```ps1
scoop bucket add scoop-adam https://github.com/sanbox-irl/scoop-adam
scoop install adam
```

If you're a **Rust programmer**, then you can install `adam` through cargo:

```sh
cargo install adam
```

## CHANGELOG AND ROADMAP

Please see the [CHANGELOG](CHANGELOG.md) for release history, and the [ROADMAP](ROADMAP.md) for planned features.

## CUSTOMIZATION

You will likely need to customize an `adam` command for most usages. To see CLI options, simply run `adam run --help`:

```txt
$ adam run --help
adam-run
Compiles, if necessary, and then runs a project

USAGE:
    adam run [FLAGS] [OPTIONS]

FLAGS:
    -h, --help            Prints help information
    -i, --ignore-cache    Ignore cache. Can use multiples times, like `-ii`. >0 disables quick recompiles, >1 disables
                          all caching
    -v, --verbosity       Verbosity level. Can use multiple times, like '-vv'. >0 disables pretty compiles, >1 enables
                          igor verbosity, >2 enables gmac verbosity
    -V, --version         Prints version information
    -y, --yyc             Uses the YYC instead of the default VM

OPTIONS:
    -c, --config <config>
            Specifies a configuration. If not passed, we use `Default` for our Config
    -g, --gms2-install-location <gms2-install-location>
            The path to your Gms2 installation. Defaults to C drive on Windows and Applications on macOS. If you use
            Steam, you will need to pass in that fullpath to the .exe, or the .app on macOS
    -o, --output-folder <output-folder>                    The relative path to the output folder. Defaults to `target`
        --yyp <yyp>                                        Specifies the target Yyp to build, if there are multiple
```

Of special note, please see `--yyc`, which will allow users to compile using the Yyc, and `-c`, which allows users to pass in a configuration. The Gms2 IDE is currently bugged, and will reset your configuration after every IDE restart; `adam` does not have this bug.

However, passing in numerous values every compile can become tiresome. To support this, users can create a config file in either `JSON` or `TOML`, where these options can be specified. To create an adam configuration file, please follow [this guide](docs/CONFIG_FILE_GUIDE.md).

## LICENSE

Dual-licensed under MIT or APACHE 2.0.
