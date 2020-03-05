// Using raw WASI syscalls for demonstration purposes.
// Use as-wasi for normal WASI Development.
// https://github.com/jedisct1/as-wasi

import {
  fd_write,
  proc_exit
} from "bindings/wasi";

function log(s: string): void {

  // Add a newline to the string
  s += "\n";

  // Convert the string into a UTF8 byte buffer
  let s_utf8_len: usize = String.UTF8.byteLength(s);
  let s_utf8 = changetype<usize>(String.UTF8.encode(s));


  // Allocate an array buffer in Wasm Memory
  let iov = changetype<usize>(new ArrayBuffer(2 * sizeof<usize>()));

  // Write the string buffer, into the allocated array buffer in Wasm Memory
  store<u32>(iov, s_utf8);
  store<u32>(iov + sizeof<usize>(), s_utf8_len);

  // Get the length of an element in an array buffer
  let iovs_len = changetype<usize>(new ArrayBuffer(sizeof<usize>()));

  // Call the WASI fd_write syscall.
  // First parameter is the file descriptor to write to. (1 -> /dev/stdout)
  fd_write(1, iov, 1, iovs_len);
}

// Entry point into our module
export function _start(): void {
  log("Hello WASI!");
}

// Handle Aborting the program on errors.
@global
export function wasi_abort(
  message: string = "",
  fileName: string = "",
  lineNumber: u32 = 0,
  columnNumber: u32 = 0
): void {
  log("Abort!");
  proc_exit(255);
}
