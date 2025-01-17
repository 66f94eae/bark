# Bark CLI

## Overview

OverWrite the code [直接调用apns接口](https://bark.day.app/#/apns?id=%e7%9b%b4%e6%8e%a5%e8%b0%83%e7%94%a8apns%e6%8e%a5%e5%8f%a3)

Bark CLI is a command-line interface tool developed in Rust for sending push notifications to iOS devices.

It is designed to be simple and easy to use, you can use it to send notification to your phone by a single command.

And it is also designed to be secure, it uses Apple Push Notification service (APNs) with JWT authentication.

## Key Features

- **Multi-Device Support**: Send notifications to multiple iOS devices simultaneously.
- **Customizable Notifications**: Set custom titles and messages for your notifications.
- **Delayed Sending**: Schedule notifications to be sent after a specified delay.
- **Secure Communication**: Utilizes Apple Push Notification service (APNs) with JWT authentication.

## Technical Details

- **Language**: Rust
- **Dependencies**:
  - `clap`: For parsing command-line arguments
  - `openssl`: For cryptographic operations and JWT token generation
  - `reqwest`: For making HTTP requests to the APNs servers
  - `toml`: For parsing configuration files
  - `serde`: For parsing TOML configuration files
- **Build Dependencies**:
  - `git2`: For git operations 
  - `chrono`: For time operations

## Installation

To install Bark CLI, you need to have Rust and Cargo installed on your system. Then, you can build the project from source:

### Build from source
```bash
git clone https://github.com/66f94eae/bark.git
cd bark
cargo build --release
```
### Install from release
```bash
# at the root of the project
cargo install --path .
```

## Usage

Use ```bark --help``` to show help info like this:

```bash
This a cli tool for send notification to your device

Usage: bark [OPTIONS] -m <MSG> -r <RECEIVER>...

Commands:
  user  alias of device token
  help  Print this message or the help of the given subcommand(s)

Options:
  -t, --title <TITLE>
          title

          [default: Notification]

  -m, --msg <MSG>
          msg content

  -r, --receiver <RECEIVER>
          send to whom in format of user1,user2...

  -d, --delay <DELAY>
          after how many seconds to send, positive number [1..]

  -l, --level <LEVEL>
          Push Interruption Level(active, timeSensitive, passive)

          [default: active]

  -b, --badge <BADGE>
          Push Badge

  -g, --group <GROUP>
          Group messages
          pushes will be displayed in groups in the notification center

  -s, --sound <SOUND>
          You can set different ringtones for the push

          [default: chime.caf]

  -i, --icon <ICON>
          Set a custom icon for the push
          the set icon will replace the default Bark icon

          [default: https://github.com/66f94eae/bark/raw/main/bot.jpg]

      --archive <ARCHIVE>
          Pass true to save the push else will not save the push
          if not passed, it will be decided according to the app's internal settings

          [possible values: true, false]

      --auto-copy <AUTO_COPY>
          Pass false to disable
          Automatically copy push content below iOS 14.5
          above iOS 14.5, you need to manually long-press the push or pull down the push

          [default: true]
          [possible values: true, false]

      --copy <COPY>
          When copying the push, specify the content to copy
          if this parameter is not provided, the entire push content will be copied

      --url <URL>
          The URL to jump to when clicking the push, supports URL Scheme and Universal Link

      --aes128
          aes128

      --aes256
          aes256

      --aes192
          aes192

      --cbc
          cbc mode

      --ecb
          ecb mode

      --gcm
          gcm mode

  -k, --key <KEY>
          encryption key

      --iv <IV>
          iv if not passed, it will be randomly generated

  -c, --config <CONFIG>
          config file in toml format

          [default: /run/bark]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

subcommands: user alias of device token
Useage: ` bark help user`
```bash
alias of device token

Usage: bark user [OPTIONS]

Options:
      --add <ADD>...  add user like "alias:device_token" ["alias1:device_token1" ...]
      --del <DEL>...  delete user like "alias1" ["alias2" ...]
      --get [<GET>]   get user like "alias"
                      if not passed, all users will be displayed
  -h, --help          Print help
```

**Note:** 
- The `-k` option is used to specify the encryption key, which is required when using the `--aes128`, `--aes192`, `--aes256`options.
- The `receiver` is a comma-separated list of *device tokens or alias* like `user1,user2,alias1...`
<br/>The program will try to find the corresponding device token based on the alias.
<br/>If the alias is not found, the program will use the `string` which you input directly.
- ![how to get your device token](https://github.com/66f94eae/bark/raw/main/device_token.png "how to get your device token")


## Example
1. send notification to user1 and user2 after 10 seconds
```bash
bark -m "hello world" -r "user1,user2" -d 10
```
2. send notification to user1 and user2 with custom title and message
```bash
bark -m "hello world" -r "user1,user2" -t "custom title"
```
3. send encrypt notification to user1 and user2, with *aes128* encryption algorithm and *gcm* mode, **iv** will randomly generted, **key,ase128,gcm** must the same as the receiver
```bash
bark -m "hello world" -r "user1,user2" -k "encrypt key" -aes128 --gcm
```
4. <a id="step4"></a> use "alis1","alias2" as alias of "device_token1","device_token2"
```bash
bark user --add "alias1:device_token1" "alias2:device_token2"
```
5. delete "alias1"
```bash
bark user --del "alias1"
```
6. get "alias1" device token
```bash
bark user --get "alias1"

output:
ALIAS     DEVICE_TOKEN 
------    -------------
alias1    device_token1
```
7. get all users
```bash
bark user --get

output:
ALIAS     DEVICE_TOKEN 
------    -------------
alias1    device_token1
alias2    device_token2
```
8. send notification use alias instead of device token
```bash
bark -m "hello world" -r "alias1,alias2"
```
9. send notification use alias and device token
<br/>if you has add alias like [step 4](#step4)
<br/>the program will send to device_token1 and device_token5 in background
```bash
bark -m "hello world" -r "alias1,device_token5"
```


## known issue
- not all param support in encrypt mode [detail in code](https://github.com/Finb/Bark/blob/master/NotificationServiceExtension/Processor/CiphertextProcessor.swift#L13)