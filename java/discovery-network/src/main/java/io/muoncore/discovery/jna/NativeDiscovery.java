package io.muoncore.discovery.jna;

import com.sun.jna.*;
import io.muoncore.Discovery;
import io.muoncore.ServiceDescriptor;
import org.scijava.nativelib.NativeLibraryUtil;
import org.scijava.nativelib.NativeLoader;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class NativeDiscovery implements Discovery {

    private DiscoveryLib discoLib;
    private Pointer instance;

    public class ServiceDescriptorInternal extends Structure {

        public String identifier;

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("identifier");
        }
    }

    public interface DiscoveryLib extends Library {
        void advertise_local_service(Pointer discoInstance, ServiceDescriptorInternal serviceDescriptor);
        void on_ready(Pointer m, Callback cb);
        List<ServiceDescriptorInternal> get_known_services(Pointer discoInstance);
        void shutdown(Pointer discoInstance);
        Pointer create(String name);
    }

    public NativeDiscovery(String libName) throws IOException {

//        NativeLoader.loadLibrary(libName);
//        NativeLibraryUtil.loadNativeLibrary(NativeDiscovery.class, libName);

        discoLib = Native.loadLibrary(libName,
                DiscoveryLib.class);
        instance = discoLib.create("WOOT BOOT");
    }

    @Override
    public List<ServiceDescriptor> getKnownServices() {
        return discoLib.get_known_services(instance)
                .stream()
                .map(serviceDescriptorInternal -> new ServiceDescriptor(null, null, null, null, null))
                .collect(Collectors.toList());
    }

    @Override
    public void advertiseLocalService(ServiceDescriptor serviceDescriptor) {
        ServiceDescriptorInternal sd = new ServiceDescriptorInternal();
        sd.identifier = serviceDescriptor.getIdentifier();
        discoLib.advertise_local_service(instance, sd);
    }

    @Override
    public void onReady(final DiscoveryOnReady discoveryOnReady) {
        discoLib.on_ready(instance, new Callback() {
            public void callback() {
                discoveryOnReady.call();
            }
        });
    }

    @Override
    public void shutdown() {
        discoLib.shutdown(instance);
    }
}
