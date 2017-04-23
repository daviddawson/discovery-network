package io.muoncore.discovery.network;

import io.muoncore.discovery.jna.NativeDiscovery;

import java.io.IOException;

public class NetworkDiscovery extends NativeDiscovery {
    public NetworkDiscovery() throws IOException {
        super("muon_discovery_net");
    }
}
