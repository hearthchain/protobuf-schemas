const PROTO_DIR: &str = "proto";

const CORE_PROTOS: &[&str] = &[
    "proto/hearth/amount.proto",
    "proto/hearth/block.proto",
    "proto/hearth/order.proto",
    "proto/hearth/recipient.proto",
    "proto/hearth/reward_share.proto",
    "proto/hearth/state_snapshot.proto",
    "proto/hearth/transaction.proto",
    "proto/hearth/transaction_state_snapshot.proto",
];

#[cfg(feature = "grpc")]
const GRPC_PROTOS: &[&str] = &[
    "proto/hearth/events/events.proto",
    "proto/hearth/events/grpc/blockchain_updates.proto",
    "proto/hearth/node/grpc/accounts_api.proto",
    "proto/hearth/node/grpc/assets_api.proto",
    "proto/hearth/node/grpc/blockchain_api.proto",
    "proto/hearth/node/grpc/blocks_api.proto",
    "proto/hearth/node/grpc/transactions_api.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={PROTO_DIR}");

    // protox parses the schemas in pure Rust, so no system protoc is required.
    let core_descriptors = protox::compile(CORE_PROTOS, [PROTO_DIR])?;
    prost_build::Config::new().compile_fds(core_descriptors)?;

    #[cfg(feature = "grpc")]
    {
        let grpc_descriptors = protox::compile(GRPC_PROTOS, [PROTO_DIR])?;

        // prost resolves well-known wrapper types used as RPC request/response types to
        // Rust primitives (google.protobuf.UInt32Value -> u32, google.protobuf.Empty -> ()).
        // tonic-prost-build is supposed to enable the matching "non-path type" allowlist via
        // `with_extended_rust_types`, but only applies it in `compile_with_config`, not in
        // `compile_fds`, so set it here explicitly.
        tonic_prost_build::NON_PATH_TYPE_ALLOWLIST.with(|allowlist| {
            *allowlist.borrow_mut() = &["()", "bool", "i32", "i64", "u32", "u64", "f32", "f64"]
        });

        // Generated into a subdirectory: the gRPC descriptor set also contains the imported
        // core schemas, and compiling them again must not overwrite the files generated above.
        let grpc_out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?).join("grpc");
        std::fs::create_dir_all(&grpc_out_dir)?;

        tonic_prost_build::configure()
            .with_extended_rust_types(true)
            .out_dir(&grpc_out_dir)
            .compile_fds(grpc_descriptors)?;
    }

    Ok(())
}
