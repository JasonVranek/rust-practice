from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libembed.dylib")

lib.process(1000000)

print("done!")

