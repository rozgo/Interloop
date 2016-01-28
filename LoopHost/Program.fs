
[<EntryPoint>]
let main argv = 
    printfn "%A" argv
    printfn "Hello world!"
    printfn "%A" (System.IO.Directory.GetCurrentDirectory())
    EvtLoop.hello_world ()
    printfn "Num from rust: %i" (EvtLoop.get_num ())
    0

