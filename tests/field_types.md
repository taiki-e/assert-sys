
A field written as `_` must still match the reference field's signedness.

```compile_fail
#[repr(C)]
struct futex_waitv {
    val: i64,
    uaddr: u64,
    flags: u32,
    __reserved: u32,
}
assert_sys::static_assert_sys_struct! {
    struct futex_waitv { val: _, uaddr: u64, flags: u32, __reserved: u32, }
}
```

A pointer with an inferred target must point to the corresponding type.

```compile_fail
#[repr(C)]
struct robust_list { next: *mut u8 }
assert_sys::static_assert_sys_struct! {
    struct robust_list { next: *mut _, }
}
```

Inferring the whole pointer type also checks its mutability.

```compile_fail
#[repr(C)]
struct robust_list { next: *const robust_list }
assert_sys::static_assert_sys_struct! {
    struct robust_list { next: _, }
}
```

A nested list that loses its only required member fails to compile.

```compile_fail,E0277
use assert_sys::sys::{robust_list, robust_list_head};
assert_sys::static_assert_sys_struct! {
    struct robust_list_head {
        list: robust_list {
            #[cfg(any())]
            next: *mut robust_list,
        },
        futex_offset: i64,
        list_op_pending: *mut robust_list,
    }
}
```
