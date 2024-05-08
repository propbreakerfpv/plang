(module
(import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))

    (memory 1)
    (export "memory" (memory 0))

    (func $println (param $msg i32)
        (local $ptr i32)
        (local $len i32)
        (i32.load (local.get $msg))
        (local.set $len)

        ;; store the length of the string in magic address 104
        ;;(i32.store (i32.const 104) (local.get $msg))
        (i32.store (i32.const 104) (local.get $len))

        ;; add 4 to the string address to get the address of the first byte
        (local.get $msg)
        (i32.const 4)
        (i32.add)
        (local.set $ptr)
        ;; store a pointer to the first byte in magic address 100
        ;;(i32.store (i32.const 100) (local.get $ptr))
        (i32.store (i32.const 100) (i32.const 4))

        (call $fd_write
            (i32.const 1) ;; file_descriptor - 1 for stdout
            (i32.const 100) ;; *iovs - The pointer to the iov array, which is stored at memory location 0
            (i32.const 1) ;; iovs_len - We're printing 1 string stored in an iov - so one.
            (i32.const 20) ;; nwritten - A place in memory to store the number of bytes written
        )
        drop
    )
    (export "println" (func $println))
(func $test
(i32.store (i32.const 0) (i32.const 11))
(i32.store (i32.const 4) (i32.const 104))
(i32.store (i32.const 5) (i32.const 101))
(i32.store (i32.const 6) (i32.const 108))
(i32.store (i32.const 7) (i32.const 108))
(i32.store (i32.const 8) (i32.const 111))
(i32.store (i32.const 9) (i32.const 32))
(i32.store (i32.const 10) (i32.const 119))
(i32.store (i32.const 11) (i32.const 111))
(i32.store (i32.const 12) (i32.const 114))
(i32.store (i32.const 13) (i32.const 108))
(i32.store (i32.const 14) (i32.const 100))

(call $println (i32.const 0)))
(func $main (export "_start")
(i32.const 5)
(i32.const 2)
(i32.gt_u)
(if

(then

(call $test )
)

(else

(i32.store (i32.const 0) (i32.const 4))
(i32.store (i32.const 4) (i32.const 116))
(i32.store (i32.const 5) (i32.const 114))
(i32.store (i32.const 6) (i32.const 117))
(i32.store (i32.const 7) (i32.const 101))

(call $println (i32.const 0))
)

)
)
)