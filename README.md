# Cars2RichPresence
Rich Presence support for Cars 2: Arcade!

this is a Pentane plugin and as such requires Pentane to be installed (you can find it [here](https://github.com/high-octane-dev/pentane)).

## installation
download the plugin from the **Releases page** (found [here](https://github.com/Flipleerr/Cars2RichPresence/releases/)) and put it in your `Pentane/Plugins` folder. then add it to your `config.toml` as shown below:
```toml
# EXAMPLE CONFIG
[plugins]
enabled_plugins = ["ArcadeEssentials.dll", "Cars2RichPresence.dll"]
```
to avoid conflicts with other plugins, it is recommended you place this plugin *below* any others you might have installed.

## building
you need cargo and rustup to compile this plugin. assuming you already have it, add the `i686-pc-windows-msvc` target like so:

```rustup add target i686-pc-windows-msvc```

then to build:

```cargo build --target i686-pc-windows-msvc --release```

alternatively, if you're on macOS or Linux you can use xwin to cross-compile:
```
cargo install xwin
rustup add target i686-pc-windows-msvc
cargo xwin build --target i686-pc-windows-msvc --release --xwin-arch x86 --xwin-include-debug-libs
```

## credits
- [RiskiVR](https://github.com/RiskiVR) - for providing the assets used for in-game states
- [MythicalBry](https://github.com/MythicalBry) - for providing the logo used for the In Menus state
- [itsmeft24](https://github.com/itsmeft24) - for helping with function hooking and other technical stuff
