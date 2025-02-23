use ichirp::HopData;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    for i in 0..10 {
        do_hop_ipv4("8.8.8.8", i, 64);
    }
     */
    let mut hops = HopData::new();
    hops.echo_request_ipv4("8.8.8.8", 1, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 2, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 2, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 2, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 3, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 3, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 3, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 4, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 4, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 4, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 5, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 5, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 5, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 6, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 6, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 6, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 6, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 7, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 7, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 7, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 8, 64)?;
    hops.echo_request_ipv4("8.8.8.8", 8, 64)?;
    hops.list_all_probes();

    //do_hop_ipv4("8.8.8.8", 1, 64);
    Ok(())
}
