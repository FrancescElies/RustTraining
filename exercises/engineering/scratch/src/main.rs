fn main() {
    // Task 1: make build.rs set these. Until then they print "unset".
    println!("git:     {}", option_env!("GIT_HASH").unwrap_or("unset"));
    println!("profile: {}", option_env!("BUILD_PROFILE").unwrap_or("unset"));
    println!("checksum: {}", scratch::checksum(b"hello"));
}
