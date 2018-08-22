## SetTimes

This was just a program I wrote in rust to see if I could write a short, comparable functionality program to something a friend of mine wrote.

```
SetTimes v1.0.0
Logan Saso <logansaso@gmail.com>
Sets system modified or accessed times.

USAGE:
    rust [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --accessed <ACCESSED>    Time in seconds since EPOCH to set as file last accessed.
    -m, --modified <MODIFIED>    Time in seconds since EPOCH to set as file last modified.

ARGS:
    <INPUT>    Sets the file to modifiy.
```
