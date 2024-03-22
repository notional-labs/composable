{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }:
    let
      devnet-root-directory = "/tmp/composable-devnet";
      RUST_LOG =
        "info,hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug";
    in {
      packages = rec {
        composable-picasso-ibc-init = pkgs.writeShellApplication {
          name = "composable-picasso-ibc-init";
          runtimeInputs = devnetTools.withBaseContainerTools;
          text = ''
            sleep 32
            mkdir --parents "/tmp/composable-devnet/composable-picasso-ibc"
            HOME="/tmp/composable-devnet/composable-picasso-ibc"
            export HOME
            RUST_LOG="${RUST_LOG},jsonrpsee_client_transport::ws=info,soketto=info,tracing::span=info,mio::poll=info,trie=info,jsonrpsee_core::client::async_client=info"
            export RUST_LOG
            cp --dereference --no-preserve=mode,ownership --force ${self'.packages.ibc-composable-to-picasso-config-1-1} "/tmp/composable-devnet/composable-picasso-ibc/config-chain-a.toml"  
            cp --dereference --no-preserve=mode,ownership --force ${self'.packages.ibc-picasso-to-composable-polkadot-config-0-0} "/tmp/composable-devnet/composable-picasso-ibc/config-chain-b.toml"  
            cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-core} "/tmp/composable-devnet/composable-picasso-ibc/config-core.toml"                
            ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-clients --config-a "/tmp/composable-devnet/composable-picasso-ibc/config-chain-a.toml" --config-b /tmp/composable-devnet/composable-picasso-ibc/config-chain-b.toml --config-core /tmp/composable-devnet/composable-picasso-ibc/config-core.toml --delay-period 10
          '';
        };

        composable-picasso-ibc-connection-init = pkgs.writeShellApplication {
          name = "composable-picasso-ibc-connection-init";
          runtimeInputs = devnetTools.withBaseContainerTools;
          text = ''
            HOME="${devnet-root-directory}/composable-picasso-ibc"
            export HOME                
            RUST_LOG="hyperspace=info,hyperspace_parachain=info,hyperspace_cosmos=info"
            export RUST_LOG      
            ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-connection --config-a ${devnet-root-directory}/composable-picasso-ibc/config-chain-a.toml --config-b ${devnet-root-directory}/composable-picasso-ibc/config-chain-b.toml --config-core ${devnet-root-directory}/composable-picasso-ibc/config-core.toml --delay-period 10
          '';
        };

        composable-picasso-ibc-channels-init = pkgs.writeShellApplication {
          name = "composable-picasso-ibc-channels-init";
          runtimeInputs = devnetTools.withBaseContainerTools;
          text = ''
            HOME="${devnet-root-directory}/composable-picasso-ibc"
            export HOME       
            RUST_LOG="${RUST_LOG}"
            export RUST_LOG
            RUST_LOG='${RUST_LOG}' ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-channel --config-a ${devnet-root-directory}/composable-picasso-ibc/config-chain-a.toml --config-b ${devnet-root-directory}/composable-picasso-ibc/config-chain-b.toml --config-core ${devnet-root-directory}/composable-picasso-ibc/config-core.toml --delay-period 10 --port-id transfer --version ics20-1 --order unordered
          '';
        };

        composable-picasso-ibc-relay = pkgs.writeShellApplication {
          name = "composable-picasso-ibc-relay";
          runtimeInputs = devnetTools.withBaseContainerTools;
          text = ''
            HOME="/tmp/composable-devnet/composable-picasso-ibc"
            export HOME
            RUST_LOG="hyperspace=info,hyperspace_parachain=info,hyperspace_cosmos=info"
            export RUST_LOG
            sed -i "s/private_key = \"\/\/Alice\"/private_key = \"\/\/Bob\"/" "/tmp/composable-devnet/composable-picasso-ibc/config-chain-a.toml"
            sed -i "s/private_key = \"\/\/Alice\"/private_key = \"\/\/Bob\"/" "/tmp/composable-devnet/composable-picasso-ibc/config-chain-b.toml"
            ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace relay --config-a /tmp/composable-devnet/composable-picasso-ibc/config-chain-a.toml --config-b /tmp/composable-devnet/composable-picasso-ibc/config-chain-b.toml --config-core /tmp/composable-devnet/composable-picasso-ibc/config-core.toml --delay-period 10
          '';
        };
      };
    };
}
