open System.Collections.Concurrent
open System.Collections.Generic
open System.Threading
open Microsoft.FSharp.NativeInterop
open System.Runtime.InteropServices

#nowarn "9"

let queue = new ConcurrentQueue<byte[]> ()

let worker () =

    let rand = new System.Random ()

    let thread = new ThreadStart (fun () ->
        Thread.Sleep 500
        let threadId = Thread.CurrentThread.ManagedThreadId
        printfn "Thread: %i waiting for work." threadId
        while true do
            let (ok, bytes) = queue.TryDequeue ()
            match ok with
            | true ->
                printfn "Got some work for thread: %i" threadId
            | false -> ()
            Thread.Sleep (int (rand.NextDouble() * 2000.0))
            ()
        )

    let thread = new Thread (thread)
    thread.Start ()
    thread


[<EntryPoint>]
let main argv = 
    printfn "%A" argv

    printfn "#### BEGIN: Sanity check ####"
    EvtLoop.Tests.run ()
    printfn "#### END: Sanity check ####"

    let threads = [0..6] |> List.map (fun i -> worker ())

    let get_array = new EvtLoop.ByteArrayFunc (fun s b ->
        let bytes : byte [] = Array.zeroCreate s
        let bPtr = (NativePtr.toNativeInt b)
        Marshal.Copy (bPtr, bytes, 0, s)
        queue.Enqueue bytes
        0 )

    EvtLoop.launch get_array

    0

