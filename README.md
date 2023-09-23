# Discorder

A cli tool for sending text or file to Discord Webhook

```txt
Usage: discorder [OPTIONS]

Options:
  -w, --webhook <WEBHOOK>  Discord Webhook URL
  -t, --text <TEXT>        A text to send
  -f, --file <FILE>        A file to send
  -c, --config <CONFIG>    Config file path [default: ./discorder.yaml]
  -h, --help               Print help
  -V, --version            Print version
```

## Installation

All [releases](
    https://github.com/sor4chi/discorder/releases
) are available on GitHub.

### eg. using `wget` and `unzip`

```sh
$ wget <URL of the release asset>
$ unzip <Downloaded file>
$ sudo install <Unzipped file> /usr/local/bin
```

## Usage

```sh
# Send a text
$ discorder -w https://discordapp.com/api/webhooks/xxx/yyy -t "Hello, World!"

# Send a file
$ discorder -w https://discordapp.com/api/webhooks/xxx/yyy -f ./hello.txt

# Send a text and a file
$ discorder -w https://discordapp.com/api/webhooks/xxx/yyy -t "Hello, World!" -f ./hello.txt

# Send from stdin
$ echo "Hello, World!" | discorder -w https://discordapp.com/api/webhooks/xxx/yyy
$ cat ./hello.txt | discorder -w https://discordapp.com/api/webhooks/xxx/yyy
```

## Config file

```yaml
webhook: https://discordapp.com/api/webhooks/xxx/yyy
```

Config load order is:

1. `--config` option
2. `./discorder.yaml`
3. `./discorder.yml`
4. `~/.config/discorder/discorder.yaml`
5. `~/.config/discorder/discorder.yaml`
