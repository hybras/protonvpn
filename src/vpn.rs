use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    net::Ipv4Addr,
    path::Path,
};

use anyhow::{Context, Result};
use askama::Template;
use util::ConnectionProtocol;

pub mod constants;
pub mod util;

#[derive(Template)] // this will generate the code...
#[template(path = "openvpn_template.j2")]
struct OpenVpnConfig {
    openvpn_protocol: ConnectionProtocol,
    serverlist: Vec<Ipv4Addr>,
    openvpn_ports: Vec<usize>,
    split: bool,
    ip_nm_pairs: Vec<IpNm>,
    ipv6_disabled: bool,
}

/// An IPv4 address and a netmask.
struct IpNm {
    ip: Ipv4Addr,
    nm: Ipv4Addr,
}

fn create_openvpn_config(
    servers: &Vec<Ipv4Addr>,
    protocol: &ConnectionProtocol,
    ports: &Vec<usize>,
    split_tunnel: &bool,
    split_tunnel_file: Option<&Path>,
    output_file: &Path,
) -> Result<()> {
    let mut ip_nm_pairs = vec![];

    if *split_tunnel {
        if let Some(path) = split_tunnel_file {
            let file = File::open(path).context("file open")?;
            let file = BufReader::new(file);
            for line in file.lines() {
                let line = line.context("line unwrap")?;
                // TODO String.split_once() once stabilized
                let tokens = line.splitn(2, "/").collect::<Vec<_>>();
                let ip_nm = match tokens.as_slice() {
                    [ip, nm] => IpNm {
                        ip: ip.parse()?,
                        nm: nm.parse()?,
                    },
                    [ip] => IpNm {
                        ip: ip.parse()?,
                        nm: "255.255.255.255".parse()?,
                    },
                    _ => {
                        continue;
                    }
                };
                ip_nm_pairs.push(ip_nm);
            }
        }
    }

    let ovpn_conf = OpenVpnConfig {
        openvpn_protocol: *protocol,
        serverlist: servers.clone(),
        openvpn_ports: ports.clone(),
        split: *split_tunnel,
        ip_nm_pairs,
        // TODO check if ipv6 is actually disabled
        ipv6_disabled: false,
    };

    let rendered = ovpn_conf.render().context("render template")?;
    let mut out = BufWriter::new(File::create(output_file)?);
    write!(out, "{}", rendered).context("Failed write")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ovpn_conf() {
        let out_path = Path::new("output.ovpn");
        let res = create_openvpn_config(
            &vec![Ipv4Addr::new(1, 1, 1, 1)],
            &ConnectionProtocol::UDP,
            &vec![1134],
            &false,
            None,
            &out_path,
        );
        println!("{:?}", res);
        assert!(res.is_ok());
    }
}
