module EvtLoop

open System.Runtime.InteropServices
open Microsoft.FSharp.NativeInterop

#nowarn "9"

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern void test_hello_world ()

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern int32 test_get_num ()

type ByteArrayFunc = delegate of int32 * nativeptr<byte> -> int32

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern void test_get_array (ByteArrayFunc)

type CombineFunc = delegate of int32 * int32 -> int32

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern int32 test_combine (CombineFunc)

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern void test_send_string (string)


let test () =

    let combine_mul = new CombineFunc (fun a b -> a * b)
    let combine_add = new CombineFunc (fun a b -> a + b)

    let get_array = new ByteArrayFunc (fun s b ->
        let bytes : byte [] = Array.zeroCreate s
        let bPtr = (NativePtr.toNativeInt b)
        Marshal.Copy (bPtr, bytes, 0, s)
        printfn "Got bytes from rust: %A" bytes
        0 )

    printfn "Hello world!"
    printfn "%A" (System.IO.Directory.GetCurrentDirectory())
    test_hello_world ()
    printfn "Num from rust: %i" (test_get_num ())
    test_get_array get_array
    printfn "Combine 2 3 *: %A" (test_combine combine_mul)
    printfn "Combine 2 3 +: %A" (test_combine combine_add)
    test_send_string "F# says hi"

