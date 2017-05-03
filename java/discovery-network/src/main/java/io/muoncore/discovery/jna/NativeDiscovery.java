package io.muoncore.discovery.jna;

import com.sun.jna.*;
import io.muoncore.Discovery;
import io.muoncore.ServiceDescriptor;
import lombok.AllArgsConstructor;
import lombok.NoArgsConstructor;
import org.scijava.nativelib.NativeLibraryUtil;
import org.scijava.nativelib.NativeLoader;

import java.io.IOException;
import java.net.URI;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.stream.Collectors;

public class NativeDiscovery implements Discovery {

    private DiscoveryLib discoLib;
    private Pointer instance;

    public static class ServiceDescriptorInternal extends Structure implements Structure.ByValue {

        public String identifier;
        public Pointer tags;
        public Pointer codecs;
        public Pointer connection_urls;
        public int tags_length;
        public int codecs_length;
        public int connection_urls_length;

        public List<String> getTags() {
            return Arrays.asList(tags.getStringArray(0, tags_length));
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList(
                    "identifier",
                    "tags",
                    "codecs",
                    "connection_urls",
                    "tags_length",
                    "codecs_length",
                    "connection_urls_length"
                    );
        }
    }

    @AllArgsConstructor
    @NoArgsConstructor
    public static class StringData extends Structure implements Structure.ByReference {

        public String text;

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("text");
        }
    }

    public interface DiscoveryLib extends Library {
        void advertise_local_service(Pointer discoInstance, ServiceDescriptorInternal sdi);

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

        sd.tags = makeStringArray(serviceDescriptor.getTags());
        sd.codecs = makeStringArray(serviceDescriptor.getCodecs());
        sd.connection_urls = makeStringArrayFromUrl(serviceDescriptor.getConnectionUrls());

        sd.tags_length = serviceDescriptor.getTags().size();
        sd.codecs_length = serviceDescriptor.getCodecs().length;
        sd.connection_urls_length = serviceDescriptor.getConnectionUrls().size();

        discoLib.advertise_local_service(instance, sd);
    }

    private Pointer makeStringArray(String[] data) {
        return new StringArray(data);
    }
    private Pointer makeStringArrayFromUrl(List<URI> data) {
        String[] stringVals = data.stream()
                .map(URI::toASCIIString).collect(Collectors.toList()).toArray(new String[data.size()]);

        return new StringArray(stringVals);
    }
    private Pointer makeStringArray(List<String> data) {
        String[] stringVals = new ArrayList<>(data).toArray(new String[data.size()]);

        return new StringArray(stringVals);
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
