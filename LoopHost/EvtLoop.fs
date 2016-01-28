module EvtLoop

open System.Runtime.InteropServices

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern void hello_world ()

[<DllImport("evtloop.dll", CallingConvention = CallingConvention.Cdecl)>]
extern int32 get_num ()
