# lnd-rust

The lnd-rust is the rust binding to the Lightning Network Daemon using
gRPC for communication.

## Requirements

Go and lnd are required to generate the additional code and build the
package.

On FreeBSD also need to `pkg install protobuf`

Also checkout the grpc-gateway to `work` if the system protobuf
install does not include this: `grpc-gateway/third_party/googleapis/*.proto`

```shell
mkdir work
cd work
git clone https://github.com/grpc-ecosystem/grpc-gateway.git
cd ..
```

## Updating

The file `protos/rpc.proto` should be synced with
`https://github.com/lightningnetwork/lnd/blob/master/lnrpc/rpc.proto`.

Tested with the Lightning Network Daemon version `0.10.0-beta.rc5`.

## Usage

First, add the following to `Cargo.toml`:

```toml
[dependencies]
lnd-rust = "0.1"
#grpc = "0.8.*"
grpc = { git = "https://github.com/stepancheg/grpc-rust" }
```

Next, create lightning client object:

```rs
    let client = {
        let host = "127.0.0.1";
        let port = 10009;
        let tls = certificate.into_tls(host.as_str()).unwrap();
        let c = grpc::ClientBuilder::new(&host, port)
            .explicit_tls(tls)
            .build()
            .unwrap();
        LightningClient::with_client(Arc::new(c))
    };
};
```

Now, it is possible to make requests:

```rs
    let req = rpc::GetInfoRequest::new();
    let resp = client
        .get_info(metadata(&macaroon_data), req)
        .join_metadata_result();
    let info = executor::block_on(resp).unwrap();
    println!("info: {:?}", info);
```

See `src/bin/main.rs` for details on how to read certificate and macaroon files.


## Running the sample program

First copy the `tls.cert` and `readonly.macaroon` from the running lnd
instance to a local `auth` directory; these will be in:
`~/.lnd/data/chain/bitcoin/testnet/` for a testnet connection.

```shell
cargo run -- auth/tls.cert auth/readonly.macaroon 127.0.0.1:10009
```

If the lnd was on another machine the ssh can be used to forward the rpc port:

```shell
ssh -i ~/.ssh/lnd-host_id_ed25519 -L 10009:localhost:10009 USER@LND_HOST
```
