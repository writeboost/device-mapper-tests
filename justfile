[env("CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER", "sudo")]
device-mapper-tests:
    cargo test -p device-mapper-tests -- --test-threads=1

[env("CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER", "sudo")]
writeboost-tests:
    cargo test -p writeboost-tests -- --test-threads=1

[env("CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER", "sudo")]
wb-command-tests:
    cargo test -p wb-command-tests -- --test-threads=1
