module EvtLoop

open System.Runtime.InteropServices

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern void hello_world ()

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern int32 get_num ()

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern int32* get_array ()

type Combine = delegate of int32 * int32 -> int32

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern int32 combine (Combine)


let test () =

    let combine_mul = new Combine (fun a b -> a * b)
    let combine_add = new Combine (fun a b -> a + b)

    printfn "Hello world!"
    printfn "%A" (System.IO.Directory.GetCurrentDirectory())
    hello_world ()
    printfn "Num from rust: %i" (get_num ())
    printfn "Array from rust: %A" (get_array ())
    printfn "Combine 2 3 *: %A" (combine combine_mul)
    printfn "Combine 2 3 +: %A" (combine combine_add)
