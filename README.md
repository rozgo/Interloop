Proof of concept, where F# manages thread workers to handle computations while hosting a native event loop by interop'n with Rust.

```
➜  Debug git:(master) ✗ mono64 --debug LoopHost.exe
[||]
#### BEGIN: Sanity check ####
Hello world!
"/Users/rozgo/Projects/Interloop/LoopHost/bin/Debug"
Hello world from rust - #2
Num from rust: 52
Got bytes from rust: [|1uy; 2uy; 3uy; 4uy; 5uy|]
Combine 2 3 *: 6
Combine 2 3 +: 5
Rust received a string: F# says hi
#### END: Sanity check ####
Thread: 4 waiting for work.
Thread: 6 waiting for work.
Thread: 7 waiting for work.
Thread: 5 waiting for work.
Thread: 8 waiting for work.
Thread: 9 waiting for work.
Thread: 3 waiting for work.
Got some work for thread: 3
Got some work for thread: 4
```
