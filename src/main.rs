mod fsext;
mod config;

use crate::config::{ Uid, FrontendConfig, BackendConfig };
use signal_hook::consts::{ SIGTERM, SIGQUIT, SIGINT };
use std::{
    env, process, thread, time::Duration,
    sync::{
        Arc,
        atomic::{ AtomicBool, Ordering }
    }
};


/// The HAProxy config dir
const CONFIG_DIR: &'static str = "/usr/local/etc/haproxy.inbox";
/// The name of the environment variable for the domains
const ENVVAR_DOMAIN: &'static str = "HAPROXY_DOMAINS";
/// The name of the environment variable for the backend address
const ENVVAR_BACKEND: &'static str = "HAPROXY_BACKEND";


pub fn main() {
    // Load environment config
    let domains_raw = env::var(ENVVAR_DOMAIN).expect("Missing environment variable for the managed domains");
    let backend = env::var(ENVVAR_BACKEND).expect("Missing environment variable for the backend address");
    let domains: Vec<_> = domains_raw.split(",")
        .map(|d| d.trim().to_string())
        .filter(|d| !d.is_empty())
        .collect();

    // Create the configs and await termination
    let uid = Uid::new(&domains);
    let backend = BackendConfig::new(&backend, &uid);
    let frontend = FrontendConfig::new(&domains, &uid);
    
    // Register flag for signals
    let termflag = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(SIGTERM, termflag.clone()).expect("Failed to register unix signal handler");
    signal_hook::flag::register(SIGQUIT, termflag.clone()).expect("Failed to register unix signal handler");
    signal_hook::flag::register(SIGINT, termflag.clone()).expect("Failed to register unix signal handler");

    // Wait until we get the termflag
    eprintln!("haproxy-autoconf is up and running...");
    loop {
        // Check if we have a term signal
        if termflag.load(Ordering::Relaxed) {
            // Drop the configs
            drop(backend);
            drop(frontend);
            
            // Exit gracefully
            eprintln!("Got signal; exiting...");
            process::exit(0);
        }
        
        // Sleep
        const SLEEP_INTERVAL: Duration = Duration::from_millis(100);
        thread::sleep(SLEEP_INTERVAL);
    }
}
