## rclip
A command line tool which supports copy a file contents to the system clipboard or copy the contents of the system clipboard to a file. 

### Macos Install 
Install from github master branch using the following command.

```bash
$ cargo install --git https://github.com/yahaa/rclip.git
```

### Ubuntu Install(22.04)
Install from github master branch using the following command.

```bash
$ sudo apt-get install cargo libxcb1-dev 
$ cargo install --git https://github.com/yahaa/rclip.git
$ # if there are some error such as /usr/bin/ld: cannot find -lxcb-render...
$ sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```


### Usage
```bash
$ rclip -h
rclip v0.1.0
yahaa, yuanzihua0@gmail.com
A cross-platform clipboard tool from command line

USAGE:
    rclip [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    copy     Copy a specify file contents to system clipboard
    help     Prints this message or the help of the given subcommand(s)
    paste    Paste the system clipboard contents to as specify file

```
