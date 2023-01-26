# mm

> **Warning**
> This executable is now at active development stage, so it may be incomplete,
> may contain errors and bugs, etc.
>
> Project documentation can be found [here][1]. Project has some additional
> repositories:
>
> - [libmm][2] - backend library
> - ...

`mm` is a simple command line program, that stores your notes on your computer.
Internally it uses `git` to track all the changes step-by-step and allow
a user to get back to any state.

## Build

1. Clone repository recursively (i.e. with submodules):

```bash
git clone --recursive https://github.com/GeorgyFirsov/mm.git
```

2. Build executable project using Cargo.

[1]: https://github.com/GeorgyFirsov/mm/tree/main/docs
[2]: https://github.com/GeorgyFirsov/libmm
