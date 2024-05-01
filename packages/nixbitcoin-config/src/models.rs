mod config_models {
    use garde::Validate;
    use serde::{Deserialize, Serialize};
    use std::option::Option;

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

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub enum PruneOptions {
        #[default]
        /// [default] Pruning disabled
        Disable,

        /// Manual pruning
        Manual,

        /// Automatic pruning at a certain blockchain size.
        ///
        /// This option requires `prune_size` to be set too.
        Automatic,
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
        ///
        /// null or ((unsigned integer, meaning >=0) or (one of "disable", "manual") convertible to it)
        #[garde(skip)]
        pub prune: Option<String>,

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
    }

    impl BitcoinDaemonService {
        /// Creates a new [`BitcoinDaemonService`].
        #[allow(clippy::too_many_arguments)]
        pub fn new(
            name: Option<String>,
            user: Option<String>,
            network: BitcoinNetwork,
            rpc_users: Option<Vec<BitcoinDaemonServiceRPCUser>>,
            rpc_port: Option<u16>,
            prune: Option<String>,
            prune_size: Option<u16>,
            port: Option<u16>,
            pid_file: Option<String>,
            package: Option<String>,
            group: Option<String>,
            extra_config: Option<String>,
            extra_cmd_line_options: Option<Vec<String>>,
            db_cache: Option<i16>,
            data_dir: Option<String>,
            config_file_path: Option<String>,
        ) -> Self {
            Self {
                name,
                user,
                network,
                rpc_users,
                rpc_port,
                prune,
                prune_size,
                port,
                pid_file,
                package,
                group,
                extra_config,
                extra_cmd_line_options,
                db_cache,
                data_dir,
                config_file_path,
            }
        }
    }

    #[cfg(test)]
    mod tests {
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
    }
}
