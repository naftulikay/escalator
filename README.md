# escalator [![Build Status][travis.svg]][travis]

A simple root privilege escalation and execution tool.

Normally `sudo` and `su` both use `fork(2)` before `execve(2)`, which yields a child process under the supervising
`sudo` process. This is usually fine, but when executing certain binaries which expect to be PID 1 (c.f. systemd),
certain assumptions are not met and systemd and/or its child services don't deal with a non-PID-1 parent.

## Usage

For this binary to do Heaven's work, set ownership on the binary to `root`, and then enable the `setuid` bit in the
file permissions. This will mean that any process that executes this binary will assume the UID of the file itself,
which we have just set to `root`. `escalator` will then start as `root`, will subsequently `setuid(2)` and `setgid(2)`
to become real `root:root`, and then will execute the binary with the given arguments.

Once all of these criteria are met, execute the binary as an unprivileged user like so:

```shell
escalator /lib/systemd/systemd --system --unit=multi-user.target
```

This will set the UID and GID to `0` and then replace itself with the first argument, called with the remaining
arguments as program arguments.

## License

Licensed at your discretion under either:

 - [Apache Software License, Version 2.0](./LICENSE-APACHE)
 - [MIT License](./LICENSE-MIT)

 [travis]: https://travis-ci.org/naftulikay/escalator
 [travis.svg]: https://travis-ci.org/naftulikay/escalator.svg?branch=master
