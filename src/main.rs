use tmap::do_hop_ipv4;
//result should be added to the
fn main() -> Result<(), Box<dyn std::error::Error>> {
    do_hop_ipv4("8.8.8.8", 2)
}
