use crate::{ CONFIG_DIR, fsext };
use sha2::{ Sha256, Digest };
use std::{
    fs,
    fmt::{ self, Display, Formatter }
};


/// A backend UID
pub struct Uid {
    /// The UID value
    value: String
}
impl Uid {
    /// Computes a UID for the given domains
    pub fn new<I, IT>(domains: I) -> Self where I: IntoIterator<Item = IT>, IT: ToString {
        // Collect and sort the domains
        let mut domains: Vec<_> = domains.into_iter().map(|d| d.to_string()).collect();
        domains.sort();

        // Compute a backend UID over the domains
        let mut sha256 = Sha256::new();
        domains.iter().for_each(|domain| {
            sha256.update(domain);
            sha256.update(",");
        });

        Self { value: format!("{:x}", sha256.finalize()) }
    }
}
impl Display for Uid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}


/// The frontend config
pub struct FrontendConfig {
    /// The config file path
    path: String
}
impl FrontendConfig {
    /// Creates a new frontend config for the given domains
    pub fn new<I, IT, U>(domains: I, uid: U) -> Self where I: IntoIterator<Item = IT>, IT: ToString, U: Display {
        // Collect the domains
        let mut domains: Vec<_> = domains.into_iter().map(|d| d.to_string()).collect();
        domains.sort();
        
        // Build the path and create the config
        let path = format!("{}/100-{}.cfg", CONFIG_DIR, &uid);
        let mut config = domains.iter().fold(String::new(), |mut config, domain| {
            config.push_str(&format!("use_backend {} if {{ ssl_fc_sni_end -i {} }}\n", &uid, &domain));
            config
        });
        config.push_str("\n");

        // Write the config file
        fsext::write_atomic(&config, &path).expect("Failed to write config file");
        Self { path }
    }
}
impl Drop for FrontendConfig {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.path) {
            eprintln!("Failed to remove config file ({}): {}", e, &self.path);
        }
    }
}


/// The backend config
pub struct BackendConfig {
    /// The config file path
    path: String
}
impl BackendConfig {
    /// Creates a new backend config for the given address
    pub fn new<B, U>(backend: B, uid: U) -> Self where B: Display, U: Display {
        // Build the path and create the config
        let path = format!("{}/200-{}.cfg", CONFIG_DIR, &uid);
        let config = format! {
            concat! {
                "backend {}\n",
                "mode http\n",
                "server {} {}\n",
                "http-request set-header X-Forwarded-Port %[dst_port]\n",
                "http-request add-header X-Forwarded-Proto https if {{ ssl_fc }}\n\n"
            },
            &uid, &uid, &backend
        };

        // Write the config file
        fsext::write_atomic(&config, &path).expect("Failed to write config file");
        Self { path }
    }
}
impl Drop for BackendConfig {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.path) {
            eprintln!("Failed to remove config file ({}): {}", e, &self.path);
        }
    }
}
