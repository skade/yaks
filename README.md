# yaks

`yaks` manages the yaks of `yakshav.es`.

## Installation

```sh
$ cargo install https://github.com/skade/yaks
```

## Usage

Set the `YAKS_DIR` environment variable to a directory containing a directory
structure similar to the one on [yakshav.es](https://github.com/skade.yakshav.es).

```
$ yaks --help
yaks 1.0
Florian Gilcher <flo@andersground.net>
manages yaks to shave

USAGE:
    yaks [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build      build all yaks
    help       Prints this message or the help of the given subcommand(s)
    list       lists yaks
    publish    send the yaks off to graze
    remove     remove a yak from the pen
    shave      shave a yak
```

## About

See more at [yaks](http://yakshav.es/yaks-1/) on [yakshav.es](http://yakshav.es).

## License

GPL V3
