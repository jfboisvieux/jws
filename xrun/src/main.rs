fn main() {
    println!(
        "User's Name            whoami::realname():    {}",
        whoami::realname(),
    );
    println!(
        "User's Username        whoami::username():    {}",
        whoami::username(),
    );
    println!(
        "User's Language        whoami::lang():        {:?}",
        whoami::lang().collect::<Vec<String>>(),
    );
    println!(
        "Device's Pretty Name   whoami::devicename():  {}",
        whoami::devicename(),
    );
    println!(
        "Device's Hostname      whoami::hostname():    {}",
        whoami::hostname(),
    );
    println!(
        "Device's Platform      whoami::platform():    {}",
        whoami::platform(),
    );
    println!(
        "Device's OS Distro     whoami::distro():      {}",
        whoami::distro(),
    );
    println!(
        "Device's Desktop Env.  whoami::desktop_env(): {}",
        whoami::desktop_env(),
    );
    println!(
        "Device's CPU Arch      whoami::arch():        {}",
        whoami::arch(),
    );
}
