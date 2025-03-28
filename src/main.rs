use ichirp::HopData;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut hops = HopData::new();
    hops.echo_request_ipv4("8.8.8.8", 1, 64)?;
    hops.list_all_probes();

    Ok(())
}
