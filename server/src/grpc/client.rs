//! gRPC client helpers implementation

use svc_storage_client_grpc::Clients;

/// Struct to hold all gRPC client connections
#[derive(Clone, Debug)]
pub struct GrpcClients {
    /// Svc-Storage clients
    pub storage: Clients,
}

impl GrpcClients {
    /// Create new GrpcClients with defaults
    pub fn default(config: crate::config::Config) -> Self {
        let storage_clients = Clients::new(config.storage_host_grpc, config.storage_port_grpc);

        GrpcClients {
            storage: storage_clients,
        }
    }
}

#[cfg(test)]
mod tests {
    use svc_storage_client_grpc::Client;

    use super::*;

    #[tokio::test]
    async fn test_grpc_clients_default() {
        let config = crate::config::Config::default();
        let clients = GrpcClients::default(config);

        let vehicle = &clients.storage.vehicle;
        println!("{:?}", vehicle);
        assert_eq!(vehicle.get_name(), "vehicle");

        let vertipad = &clients.storage.vertipad;
        println!("{:?}", vertipad);
        assert_eq!(vertipad.get_name(), "vertipad");

        let vertiport = &clients.storage.vertiport;
        println!("{:?}", vertiport);
        assert_eq!(vertiport.get_name(), "vertiport");
    }
}
