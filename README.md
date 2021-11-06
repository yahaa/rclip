## rclip
A command line tool which support copy a file contents to the system clipboard or copy the contents of the system clipboard to a file. 

### Install 
Install from github master branch using the following command.
```bash
$ cargo install --git https://github.com/yahaa/rclip.git
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