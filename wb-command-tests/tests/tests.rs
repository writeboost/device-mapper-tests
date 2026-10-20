use device_mapper_tests::*;

use cmd_lib::run_cmd;

#[test]
fn test_create() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(writeboost-cli create wbdev $p1 $p2 --reformat --read_cache_threshold=4 --writeback_threshold=70 --nr_max_batched_writeback=32 --update_sb_record_interval=3600 --sync_data_interval=3600 --write_around_mode).unwrap();
    run_cmd!(writeboost-cli remove wbdev).unwrap();
}

#[test]
fn test_recreate() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(writeboost-cli create wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(writeboost-cli remove wbdev).unwrap();

    run_cmd!(writeboost-cli create wbdev $p1 $p2).unwrap();
    run_cmd!(dd if=/dev/zero of=/dev/mapper/wbdev oflag=direct bs=1M count=50).unwrap();
    run_cmd!(writeboost-cli remove wbdev).unwrap();
}

#[test]
fn test_remove() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(writeboost-cli create wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(writeboost-cli remove wbdev --noflush --nowriteback).unwrap();
}

#[test]
fn test_check() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(writeboost-cli create wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(dd if=/dev/zero of=/dev/mapper/wbdev oflag=direct bs=1M count=50).unwrap();

    run_cmd!(writeboost-cli check $p2 --segid=1).unwrap();

    run_cmd!(writeboost-cli remove wbdev --nowriteback).unwrap();
}

#[test]
fn test_dump() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(writeboost-cli create wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(dd if=/dev/zero of=/dev/mapper/wbdev oflag=direct bs=1M count=50).unwrap();

    run_cmd!(writeboost-cli dump $p2 1).unwrap();

    run_cmd!(writeboost-cli remove wbdev).unwrap();
}

#[test]
fn test_meta() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(writeboost-cli create wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(dd if=/dev/zero of=/dev/mapper/wbdev oflag=direct bs=1M count=50).unwrap();

    run_cmd!(writeboost-cli meta $p2 0).unwrap();
    run_cmd!(writeboost-cli meta $p2 1).unwrap();

    run_cmd!(writeboost-cli remove wbdev).unwrap();
}

#[test]
fn test_status() {
    let mut env = env();
    let fast = env.alloc_device(Sector::MB(100));
    let p1 = fast.path();
    let slow = env.alloc_device(Sector::MB(50));
    let p2 = slow.path();
    run_cmd!(writeboost-cli create wbdev $p1 $p2 --reformat).unwrap();
    run_cmd!(dd if=/dev/zero of=/dev/mapper/wbdev oflag=direct bs=1M count=50).unwrap();

    run_cmd!(dmsetup status wbdev | writeboost-cli status).unwrap();

    run_cmd!(writeboost-cli remove wbdev --nowriteback).unwrap();
}
