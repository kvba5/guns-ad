# 🔫 guns-ad

[![GitHub Release](https://img.shields.io/github/v/release/kvba5/guns-ad?label=download)](https://github.com/kvba5/guns-ad/releases/latest)
![GitHub Repo stars](https://img.shields.io/github/stars/kvba5/guns-ad)
![GitHub forks](https://img.shields.io/github/forks/kvba5/guns-ad)

**⚠️ WARNING: The tool can be considered as "Self-bot" or "User-bot" which is against [Discord's TOS](https://dis.gd/tos)! Please use at your own risk! I'm not responsible for any account suspensions caused by this tool or any misuse of it.**  

Simple CLI tool for automating the message sending on `#bio-links` channel on [guns.lol](https://guns.lol/guns) Discord server

**The tool does not require any dependencies on any of the platforms!** I used Rust just for this reason, because a lot of people have problem with installing let's say Node.js or Python. (And also is more efficient performance-wise)

## ❔ Usage
At first run of the script you will see new file `config.toml`, this file includes anything you can customize for your own usage. [See CONFIG.md for all info about current config!](/CONFIG.md)  

After you fill everything up, just run the command with this command:
```bash
$ ./guns-ad [optional arguments]
```
(Existing command arguments can also be found in [CONFIG.md](/CONFIG.md))

## 🏗️ Building
Building is quite simple but might require you to install additional tools.  

### Requirements:
- [Rust Tools](https://www.rust-lang.org/), you might also have to [add specific targets](https://stackoverflow.com/a/53210209) if you want to build cross-platform.
- `gcc`/`build-essentials` [depending on a platform](https://stackoverflow.com/a/66598982)
- OpenSSL library *(not sure if it's needed on Windows)*
  - `libssl-dev` on Ubuntu/Debian
  - `openssl-devel` on Fedora/CentOS

### Building process:
If you're on Linux/Mac you should be able to use the `build.sh` left in repo. It will install both Windows and Linux version of the tool.  
If you're not on Linux or your OS doesn't support `.sh` files you should be able to run command below to build the tool for your OS:
```bash
$ cargo build --release
```
The final file will show up in `/target/<OS version>/release/guns-ad` if built through `build.sh` or `/target/release/guns-ad` if built through the command above.

## 📰 License
The tool is running under [MIT](/LICENSE) license but any credit is highly appreciated! ❤️
