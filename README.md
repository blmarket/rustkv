RustKV
------
    
Proof-of-concept lambda service implementation of Write-Once-Read-Many(WORM)
database.

This shows usage of rust to fetch shared rocksdb and serve traffic to provide
key-value storage using JSON protocol. Theoretically it's highly scalable as EFS
volume can be shared across multiple instances and all instances won't block
each other.

## Why?

It should cost near to $0 with little or no usage(except EFS cost, which
depends on your database size), and should be able to serve 10000+ TPS, with
linear scalability.

## Performance

Performance measured within lambda is ~1ms per invocation (after warm up) -
pretty good for PoC implementation, huh? Cold start can take up to 500ms due to
database initialization.

Note: no stress testing was done so performance under stress can be different.

## Build

Build tool assumes cross build tools are available under `/opt/cross`, you can
find source from [here](ttps://github.com/richfelker/musl-cross-make).
Additinoal configuration (put this for `config.mak`)

```
TARGET = x86_64-linux-musl
OUTPUT = /opt/cross
```

Also you may need to build musl with `CFLAGS=-fPIE` or final link will fail.

## Deploy

`make deploy` and upload rust.zip to lambda function code.

## Run

Runtime assumes rocksdb is available under `/mnt/db` from lambda function. 

## LICENSE

This application is licensed under AGPL-3.0-or-later. See COPYING for details.
