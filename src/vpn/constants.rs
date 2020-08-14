use dirs::home_dir;
use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

lazy_static! {
    static ref USER: String = env::var("USER").unwrap();
    static ref CONFIG_DIR: PathBuf = home_dir().unwrap();
    static ref CONFIG_FILE: PathBuf = {
        let mut dir = CONFIG_DIR.clone();
        dir.push("pvpn-cli.cfg");
        dir
    };
    static ref SERVER_INFO_FILE: PathBuf = {
        let mut path = CONFIG_DIR.clone();
        path.push("serverinfo.json");
        path
    };
    static ref SPLIT_TUNNEL_FILE: PathBuf = {
        let mut path = CONFIG_DIR.clone();
        path.push("split_tunnel.txt");
        path
    };
    static ref OVPN_FILE: PathBuf = {
        let mut path = CONFIG_DIR.clone();
        path.push("connect.ovpn");
        path
    };
    static ref PASSFILE: PathBuf = {
        let mut path = CONFIG_DIR.clone();
        path.push("pvpnpass");
        path
    };
}
