# 🔧 guns-ad Configuration
This file serves purpose as help for how to customize `guns-ad` tool.

## Config file
By default, config file resides in the same directory as the executable as `config.toml`, but you can change location of it using `-c`/`--config` argument followed by path to the config file.

| Key | Description | Required | Default |
|:---:|:---:|:---:|:---:|
| `token` | Token of your account. It will be used to send message on your behalf.<br><br>If you're Vencord user you can get your token with `findByProps("getToken").getToken()` in console. | `✅` | `-` |
| `msg_path` | Path to the text file including message bot will send! | `✅` | `"msg.txt"` |

## Arguments
Arguments are provided after the `./guns-ad` part of command.

| Name(s) | Description | Required | Default |
|:---:|:---:|:---:|:---:|
| `--config` `-c` | Location of your config file. | `❌` | `config.toml` |
| `--channel` | Custom channel ID the message gonna be sent to.<br><br>Should be used only when default value is deprecated and doesn't work. | `❌` | `1145771692099121206` |
| `--force-react` | Enables default Guns message reaction even when using custom channel ID, you might consider having this off when having no nitro. | `❌` | `❌` |
| `--reaction` | Valid reaction string to use instead of default Upvote one (useful when having `--force-react` set).<br><br>**Valid strings:**<br>Emoji: `😂` (just an emoji)<br>Custom: `name:id`<br>Animated Custom: `a:name:id` | `❌` | `upvote:1185979066466181162` |

You can also check arguments by using `-h` or `--help`