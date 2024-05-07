(module
    (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))

    (memory 1)
    (export "memory" (memory 0))

    (func $println (param $msg i32)
        (i32.store (i32.const 32) (i32.const 65))
        (i32.store (i32.const 33) (i32.const 10))

        (i32.store (i32.const 24) (i32.const 32))
        (i32.store (i32.const 28) (i32.const 2))


        (call $fd_write
            (i32.const 1) ;; file_descriptor - 1 for stdout
            (i32.const 24) ;; *iovs - The pointer to the iov array, which is stored at memory location 0
            (i32.const 2) ;; iovs_len - We're printing 1 string stored in an iov - so one.
            (i32.const 20) ;; nwritten - A place in memory to store the number of bytes written
        )
        drop
    )
    (export "println" (func $println))
)

