extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &[
            "src/gogoproto/gogo.proto",
            "src/kardiachain/blockchain/types.proto",
            "src/kardiachain/consensus/types.proto",
            "src/kardiachain/consensus/wal.proto",
            "src/kardiachain/crypto/keys.proto",
            "src/kardiachain/crypto/proof.proto",
            "src/kardiachain/evidence/types.proto",
            "src/kardiachain/p2p/conn.proto",
            "src/kardiachain/p2p/pex.proto",
            "src/kardiachain/p2p/types.proto",
            "src/kardiachain/state/types.proto",
            "src/kardiachain/txpool/types.proto",
            "src/kardiachain/types/block.proto",
            "src/kardiachain/types/canonical.proto",
            "src/kardiachain/types/events.proto",
            "src/kardiachain/types/evidence.proto",
            "src/kardiachain/types/params.proto",
            "src/kardiachain/types/types.proto",
            "src/kardiachain/types/validator.proto",
        ],
        &["src", "/usr/local/include/"],
    )
    .unwrap();
}

// fn config() -> prost_build::Config {
//     let mut config = prost_build::Config::new();
//     config.bytes(&["."]);
//     config
// }

// fn make_protos(protos: &[&str]) {
//     tonic_build::configure()
//         .compile_with_config(config(), protos, &["."])
//         .unwrap();
// }

// fn main() {
//     let protos = vec![
//         "kardiachain/blockchain/types.proto",
//         "kardiachain/consensus/types.proto",
//         "kardiachain/consensus/wal.proto",
//         "kardiachain/crypto/keys.proto",
//         "kardiachain/crypto/proof.proto",
//         "kardiachain/evidence/types.proto",
//         "kardiachain/libs/bits/types.proto",
//         "kardiachain/p2p/conn.proto",
//         "kardiachain/p2p/pex.proto",
//         "kardiachain/p2p/types.proto",
//         "kardiachain/state/types.proto",
//         "kardiachain/txpool/types.proto",
//         "kardiachain/types/block.proto",
//         "kardiachain/types/canonical.proto",
//         "kardiachain/types/events.proto",
//         "kardiachain/types/evidence.proto",
//         "kardiachain/types/params.proto",
//         "kardiachain/types/types.proto",
//         "kardiachain/types/validator.proto",
//     ];
//     make_protos(&protos);
// }
