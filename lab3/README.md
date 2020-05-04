# Lab3

## Build

The project is built by CMake.

`mchat_mt` target is the multithread version and `mchat_mp` one is the multiplex version.

To build the project, for example, you can use:

```bash
mkdir build && cd build
cmake .. && make
```

This will build the two `mchat_mt` and `mchat_mp` programs.
Now you can run them to test.

## Implementation

### Multithread

The version uses C++ `std::thread` to create both recv thread and send thread.
Every socket has its own queue.
Recv thread recv and push msg into the queue of other sockets with notifying the respective send thread.
Send thread wakes up and check predicate and send the msg.

### Multiplex

The version uses Linux `epoll` to wait an action.
In an infinite loop, first check if any socket requires to send msg.
Then if true, wait income nonblockingly and then perform sending, or wait income until an income comes up.
