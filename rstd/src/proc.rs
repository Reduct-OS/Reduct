pub fn exit(code: usize) -> isize {
    syscall!(sc::nr::EXIT, code)
}

pub fn fork() -> isize {
    syscall!(sc::nr::FORK)
}

pub fn vfork() -> isize {
    syscall!(sc::nr::VFORK)
}

pub fn wait4(pid: usize) -> isize {
    syscall!(sc::nr::WAIT4, pid)
}

pub fn r#yield() -> isize {
    syscall!(sc::nr::SCHED_YIELD)
}
