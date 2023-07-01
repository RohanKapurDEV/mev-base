use tonic_build::configure;

fn main() {
    configure()
        .compile(
            &[
                "protos/geyser.proto",
                "protos/confirmed_block.proto",
                "protos/transaction_by_addr.proto",
            ],
            &["protos"],
        )
        .unwrap();
}
