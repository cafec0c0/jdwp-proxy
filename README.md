# JDWP Proxy

A simple, Proxy for visualising JDWP packets.

This project was written for a talk on JDWP, and in order to demonstrate what packets get sent between a debugger and debuggee, I created this simple tool.

When first learning JDWP myself, I used Wireshark, and if you are in an environment where you can install Wireshark, I would highly recommend that.

## Compiling
To compile the release binary, run:
```
cargo build --release
```
This will produce a binary at target/release/jdwp-proxy.

## Usage
Note: The debuggee must have `libjdwp` attached and be in server mode (server=y). 
Additionally, the JVM has to be running before starting the proxy.

To run with defaults, run:
```
./jdwp-proxy
```
This will connect to the debuggee (JVM) on port 8000, and listen for incoming connections on port 8001.

To view all options, run:
```
./jdwp-proxy --help
```

## Error handling
The proxy doesn't currently handle errors gracefully, if you attempt to connect to a closed port, it will panic.

The proxy will not attempt to reconnect if the VM dies, and it will listen for new connections if the debugger disconnects.
