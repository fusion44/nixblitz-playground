use alejandra::format;

use crate::utils::{self};
use garde::Validate;
use serde::{Deserialize, Serialize};

/// Represents all available Bitcoin network options
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum BitcoinNetwork {
    #[default]
    /// [default] The mainnet network
    Mainnet,

    /// The testnet network
    Testnet,

    /// The regtest network
    Regtest,

    /// The signet network
    Signet,
}

/// Prune options for a blockchain.
///
/// This enum defines the different pruning strategies that can be used.
/// # Variants
/// - Disable: Pruning is disabled. This is the default option.
/// - Manual: Pruning is performed manually by the user.
/// - Automatic: Pruning is performed automatically when the blockchain reaches a certain size. ///
/// # Examples
/// ```
/// use nixbitcoin_config::bitcoind::PruneOptions;
///
/// let options = PruneOptions::Automatic { field: 1024 };
/// ```
#[derive(Debug, Validate, Default, Serialize, Deserialize, PartialEq)]
pub enum PruneOptions {
    #[default]
    /// [default] Pruning disabled
    Disable,

    /// Manual pruning.
    ///
    /// The user is responsible for pruning the blockchain via RPC.
    Manual,

    /// Automatic pruning at a certain blockchain size.
    ///
    /// * Only active if prune is set to automatic.
    /// * Must be at least 551 MiB.
    /// * The `field` represents the size in MiB at which automatic pruning should occur.
    Automatic {
        /// The size in MiB at which automatic pruning should occur.
        ///
        /// This field must be at least 551 MiB.
        #[garde(range(min = 551))]
        prune_at: u32,
    },
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct BitcoinDaemonServiceRPCUser {
    /// Password HMAC-SHA-256 for JSON-RPC connections. Must be a string of the format <SALT-HEX>$<HMAC-HEX>.

    /// Tool (Python script) for HMAC generation is available here:
    /// https://github.com/bitcoin/bitcoin/blob/master/share/rpcauth/rpcauth.py
    #[garde(pattern("[0-9a-f]+\\$[0-9a-f]{64}"))]
    pub password_hmac: String,

    /// Username for JSON-RPC connections.
    #[garde(length(min = 3))]
    pub name: String,
}

#[derive(Debug, Validate, Serialize, Deserialize, Default)]
pub struct BitcoinDaemonService {
    /// The name of the instance.
    #[garde(skip)]
    pub name: Option<String>,

    /// The user as which to run bitcoind.
    #[garde(length(min = 3))]
    pub user: Option<String>,

    /// Whether to use the testnet instead of mainnet.
    #[garde(skip)]
    pub network: BitcoinNetwork,

    /// RPC user information for JSON-RPC connections.
    #[garde(skip)]
    pub rpc_users: Option<Vec<BitcoinDaemonServiceRPCUser>>,

    /// Override the default port on which to listen for JSON-RPC connections.
    #[garde(range(min = 1024, max = 65535))]
    pub rpc_port: Option<u16>,

    /// Whether to prune the node
    // #[garde(custom(_check_prune(&self)))]
    #[garde(skip)]
    pub prune: Option<PruneOptions>,

    /// The size in MiB at which the blockchain on disk will be pruned.
    ///
    /// * Only active if prune is set to automatic
    /// * Must be at least 1 MiB
    #[garde(range(min = 1))]
    pub prune_size: Option<u16>,

    /// Override the default port on which to listen for connections.
    #[garde(range(min = 1024, max = 65535))]
    pub port: Option<u16>,

    /// Location of bitcoind pid file.
    #[garde(skip)]
    pub pid_file: Option<String>,

    /// The bitcoind package to use.
    #[garde(skip)]
    pub package: Option<String>,

    /// The group ta which to run bitcoind.
    #[garde(skip)]
    pub group: Option<String>,

    /// Additional configurations to be appended to bitcoin.conf
    /// Strings concatenated with "\n"
    /// # Example
    ///
    /// ''
    /// par=16
    /// rpcthreads=16
    /// logips=1
    /// ''
    #[garde(skip)]
    pub extra_config: Option<String>,

    /// Extra command line options to pass to bitcoind. Run bitcoind –help to list all available options.
    #[garde(skip)]
    pub extra_cmd_line_options: Option<Vec<String>>,

    /// Override the default database cache size in MiB.
    /// Integer between 4 and 16384 (both inclusive)
    #[garde(range(min = 4, max = 16384))]
    pub db_cache: Option<i16>,

    /// The data directory for bitcoind.
    #[garde(skip)]
    pub data_dir: Option<String>,

    /// The configuration file path to supply bitcoind.
    #[garde(skip)]
    pub config_file_path: Option<String>,

    /// Whether to enable the tx index
    #[garde(skip)]
    pub tx_index: Option<bool>,
}

impl BitcoinDaemonService {
    pub fn render(&self) -> (format::Status, String) {
        if let Err(e) = self.validate(&()) {
            panic!("invalid config: {e}")
        }

        let mut res = utils::AutoLineString::from("{");
        res.push_line("services.bitcoin = {");
        res.push_line("enable = true;");
        match self.network {
            // we could omit mainnet as it is default, but we'll add the field for clarity
            BitcoinNetwork::Mainnet => res.push_line("mainnet = true;"),
            BitcoinNetwork::Testnet => res.push_line("testnet = true;"),
            BitcoinNetwork::Regtest => res.push_line("regtest = true;"),
            BitcoinNetwork::Signet => res.push_line("signet = true;"),
        };
        match &self.rpc_users {
            Some(value) if value.len() == 1 => {
                res.push_line(format!("rpc.users.{} = {{", value[0].name).as_str());
                res.push_line(format!("name = \"{}\"", value[0].name).as_str());
                res.push_line(format!("passwordHMAC = \"{}\"", value[0].password_hmac).as_str());
                res.push_line("};");
            }
            Some(value) if value.len() > 1 => {
                for u in value {
                    res.push_line("rpc.users = {");
                    res.push_line(format!("{} = {{", u.name).as_str());
                    res.push_line(format!("passwordHMAC = \"{}\"", u.password_hmac).as_str());
                    res.push_line("};};");
                }
            }
            Some(_) => {}
            None => {}
        }
        if let Some(v) = &self.rpc_port {
            res.push_line(format!("rpc.port = {};", v).as_str());
        }
        if let Some(v) = &self.user {
            res.push_line(format!("user = \"{}\";", v).as_str());
        }
        if let Some(v) = &self.prune {
            match v {
                PruneOptions::Disable => {}
                PruneOptions::Manual => res.push_line("prune = 1;"),
                PruneOptions::Automatic { prune_at: field } => {
                    res.push_line(format!("prune = {};", field).as_str())
                }
            }
        }

        res.push_line("};}");

        format::in_memory("<convert bitcoind>".to_string(), res.to_string())
    }
}

fn _get_network_test_string(network: &BitcoinNetwork) -> String {
    format::in_memory(
        "".to_string(),
        match network {
            BitcoinNetwork::Mainnet => {
                "{ services.bitcoin = { enable = true; mainnet = true; }; }".to_string()
            }
            BitcoinNetwork::Testnet => {
                "{ services.bitcoin = { enable = true; testnet = true; }; }".to_string()
            }
            BitcoinNetwork::Regtest => {
                "{ services.bitcoin = { enable = true; regtest = true; }; }".to_string()
            }
            BitcoinNetwork::Signet => {
                "{ services.bitcoin = { enable = true; signet = true; }; }".to_string()
            }
        },
    )
    .1
}

#[cfg(test)]
mod tests {
    use garde::rules::contains::Contains;

    use crate::bitcoind::_get_network_test_string;

    use super::*;

    #[test]
    fn test_bitcoin_daemon_service_creation() {
        let service = BitcoinDaemonService {
            name: Some("TestInstance".to_string()),
            user: Some("testuser".to_string()),
            port: Some(8333),
            rpc_port: Some(9333),
            ..BitcoinDaemonService::default()
        };

        assert_eq!(service.name, Some("TestInstance".to_string()));
        assert_eq!(service.user, Some("testuser".to_string()));
        assert_eq!(service.rpc_port, Some(9333));
        assert_eq!(service.port, Some(8333));
        assert_eq!(service.network, BitcoinNetwork::Mainnet);
        assert!(service.rpc_users.is_none());
        assert!(service.prune.is_none());
        assert!(service.pid_file.is_none());
        assert!(service.package.is_none());
        assert!(service.group.is_none());
        assert!(service.extra_config.is_none());
        assert!(service.extra_cmd_line_options.is_none());
        assert!(service.db_cache.is_none());
        assert!(service.data_dir.is_none());
        assert!(service.config_file_path.is_none());
    }

    #[test]
    fn test_render_full() {
        let service = BitcoinDaemonService {
            name: Some("TestInstance".to_string()),
            user: Some("testuser".to_string()),
            port: Some(8333),
            rpc_port: Some(9333),
            prune: Some(PruneOptions::Automatic { prune_at: 1024 }),
            ..BitcoinDaemonService::default()
        };

        println!("{}", service.render().1);

        // assert!(false);
    }

    #[test]
    fn test_prune_default_options() {
        let service = BitcoinDaemonService {
            prune: None,
            ..BitcoinDaemonService::default()
        };

        let res = service.validate(&());
        assert!(res.is_ok());
        let nix = service.render().1;
        assert!(!nix.validate_contains("prune ="));
    }

    #[test]
    fn test_prune_manual_options() {
        let service = BitcoinDaemonService {
            prune: Some(PruneOptions::Manual),
            ..BitcoinDaemonService::default()
        };

        let res = service.validate(&());
        assert!(res.is_ok());
        let nix = service.render().1;
        assert!(nix.validate_contains("prune = 1;"));
    }

    #[test]
    fn test_prune_automatic_options() {
        let service = BitcoinDaemonService {
            prune: Some(PruneOptions::Automatic { prune_at: 1024 }),
            ..BitcoinDaemonService::default()
        };

        let res = service.validate(&());
        assert!(res.is_ok());
        let nix = service.render().1;
        assert!(nix.validate_contains("prune = 1024;"));

        let service_nok = BitcoinDaemonService {
            prune: Some(PruneOptions::Automatic { prune_at: 234 }),
            ..BitcoinDaemonService::default()
        };
        let res = service_nok.validate(&());
        assert!(res.is_err());
    }

    #[test]
    fn test_render_network() {
        for network in [
            BitcoinNetwork::Mainnet,
            BitcoinNetwork::Testnet,
            BitcoinNetwork::Regtest,
            BitcoinNetwork::Signet,
        ] {
            let nix = _get_network_test_string(&network);
            let service = BitcoinDaemonService {
                network,
                ..Default::default()
            };

            let (status, res) = service.render();

            assert!(!matches!(status, format::Status::Error(_)));
            assert_eq!(res.trim(), nix.trim());
        }
    }
}
