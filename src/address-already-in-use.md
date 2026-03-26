# Troubleshooting: Address already in use

When you try to run your application you might encounter this error

```
thread 'main' (81033) panicked at src/main.rs:17:10:
called `Result::unwrap()` on an `Err` value: Os { code: 98, kind: AddrInUse, message: "Address already in use" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This means that there is another application running on the same port. (port 3000) It might be that you have already tried to run an example in a different window and you have not shut it down.

On Linux or Mac it might be that you used Ctrl-Z to stop the program. Which actually only suspends it, but keeps the port used.

The solution is to either find the other instance and close it or to launch this instance on a different port.

You can do the latter by editing the `main.rs` file and replacing the port number.

There are a number of tools that can help you find the other application using this port. See the tools:

## Linux

```
netstat -nlp | grep 3000
```

## Linux and macOS

```
lsof -P -i :3000
```

## Windows

```
netstat -ano | findstr 3000
```

