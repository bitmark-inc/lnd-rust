#![forbid(unsafe_code)]

use lnd_rust::macaroon_data::MacaroonData;
use lnd_rust::rpc_grpc::LightningClient;
use lnd_rust::tls_certificate::TLSCertificate;

use lnd_rust::rpc;

use futures::executor;

use grpc::RequestOptions;

fn main() {
    use grpc::ClientStub;
    use std::{net::SocketAddr, sync::Arc};

    println!("lnd-rust main");

    if std::env::args().len() < 4 {
        println!(
            "Usage: cargo run -- %path_to_cert% %path_to_macaroon% %socket e.g. 127.0.0.1:10009%"
        );
        return;
    }

    let certificate = {
        let cert_filename = std::env::args().into_iter().skip(1).next().unwrap();
        TLSCertificate::from_path(cert_filename).unwrap()
    };

    let macaroon_data = {
        let macaroon_file_path = std::env::args().into_iter().skip(2).next().unwrap();
        MacaroonData::from_file_path(macaroon_file_path).unwrap()
    };

    let client = {
        let default = "127.0.0.1:10009";
        let socket_addr_string = std::env::args()
            .into_iter()
            .skip(3)
            .next()
            .unwrap_or(default.to_owned());
        let socket_addr: SocketAddr = socket_addr_string.parse().unwrap();
        let host = socket_addr.ip().to_string();
        let port = socket_addr.port();

        let tls = certificate.into_tls(host.as_str()).unwrap();
        let c = grpc::ClientBuilder::new(&host, port)
            .explicit_tls(tls)
            .build()
            .unwrap();

        println!("connect to IP: {}  port: {}", host, port);

        LightningClient::with_client(Arc::new(c))
    };

    let metadata = |macaroon_data: &MacaroonData| RequestOptions {
        metadata: macaroon_data.metadata(),
        cachable: false,
    };

    let req = rpc::GetInfoRequest::new();
    let resp = client
        .get_info(metadata(&macaroon_data), req)
        .join_metadata_result();

    let info = executor::block_on(resp).unwrap();

    println!("info: {:?}", info);

    let wallet_req = rpc::WalletBalanceRequest::new();
    let wallet_resp = client
        .wallet_balance(metadata(&macaroon_data), wallet_req)
        .join_metadata_result();

    let w = executor::block_on(wallet_resp).unwrap().1;

    println!("wallet: {:?}\n", w);
}
