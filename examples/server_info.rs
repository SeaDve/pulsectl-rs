fn main() {
    let mut handler =
        pulsectl::controllers::SourceController::create().expect("Failed to create handler");
    let server_info = handler
        .get_server_info()
        .expect("Failed to get server info");

    // Get informations similar to the output of `pactl info` command
    println!("User Name: {:?}", server_info.user_name);
    println!("Host Name: {:?}", server_info.host_name);
    println!("Server Name: {:?}", server_info.server_name);
    println!("Server Version: {:?}", server_info.server_version);
    println!(
        "Default Sample Specification: {:?}",
        server_info.sample_spec
    );
    println!("Default Channel Map: {:?}", server_info.channel_map);
    println!("Default Sink: {:?}", server_info.default_sink_name);
    println!("Default Source: {:?}", server_info.default_source_name);
    println!("Cookie: {:?}", server_info.cookie);
}
