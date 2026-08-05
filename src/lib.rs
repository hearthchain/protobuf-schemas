// The generated code triggers clippy lints that are out of our control.
#![allow(clippy::large_enum_variant)]

// Re-export prost (and tonic with the `grpc` feature), so that users of this library
// can use the same versions seamlessly.
pub use prost;

#[cfg(feature = "grpc")]
pub use tonic;

pub mod hearth {
    include!(concat!(env!("OUT_DIR"), "/hearth.rs"));

    #[cfg(feature = "grpc")]
    pub mod events {
        include!(concat!(env!("OUT_DIR"), "/grpc/hearth.events.rs"));

        pub mod grpc {
            include!(concat!(env!("OUT_DIR"), "/grpc/hearth.events.grpc.rs"));
        }
    }

    #[cfg(feature = "grpc")]
    pub mod node {
        pub mod grpc {
            include!(concat!(env!("OUT_DIR"), "/grpc/hearth.node.grpc.rs"));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn core_messages_are_generated() {
        let _ = crate::hearth::Block::default();
        let _ = crate::hearth::SignedTransaction::default();
        let _ = crate::hearth::BlockSnapshot::default();
        let _ = crate::hearth::EndorseBlock::default();
    }

    #[cfg(feature = "grpc")]
    #[test]
    fn grpc_services_are_generated() {
        let _ = std::any::type_name::<
            crate::hearth::node::grpc::blocks_api_client::BlocksApiClient<
                tonic::transport::Channel,
            >,
        >();
        let _ = std::any::type_name::<
            crate::hearth::events::grpc::blockchain_updates_api_client::BlockchainUpdatesApiClient<
                tonic::transport::Channel,
            >,
        >();
    }
}
