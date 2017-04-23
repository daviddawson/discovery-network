package io.muoncore.discovery.network;

import io.muoncore.Discovery;
import io.muoncore.config.AutoConfiguration;
import io.muoncore.discovery.DiscoveryFactory;
import lombok.extern.slf4j.Slf4j;

import java.util.Properties;

@Slf4j
public class NetworkDiscoveryFactory implements DiscoveryFactory {
    @Override
    public Discovery build(Properties properties) {
        NetworkDiscovery discovery = null;
        try {
            discovery = new NetworkDiscovery();
        } catch (Exception e) {
            log.info("Error creating Network discovery", e);
        }
        return discovery;
    }

    @Override
    public void setAutoConfiguration(AutoConfiguration autoConfiguration) {

    }
}
