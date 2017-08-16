package io.muoncore.discovery.jna;

import com.sun.jna.*;
import io.muoncore.Discovery;
import io.muoncore.InstanceDescriptor;
import io.muoncore.ServiceDescriptor;
import lombok.AllArgsConstructor;
import lombok.NoArgsConstructor;
import org.scijava.nativelib.NativeLibraryUtil;
import org.scijava.nativelib.NativeLoader;

import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.util.*;
import java.util.stream.Collectors;

public class NativeDiscovery implements Discovery {

    private DiscoveryLib discoLib;
    private Pointer instance;

    public static class ServiceDescriptorInternal extends Structure {
        @NoArgsConstructor
        public static class ByValue extends ServiceDescriptorInternal implements Structure.ByValue {}
        @NoArgsConstructor
        public static class ByReference extends ServiceDescriptorInternal implements Structure.ByReference {}

        public String id;
        public String identifier;
        public Pointer tags;
        public Pointer codecs;
        public Pointer connection_urls;
        public int tags_length;
        public int codecs_length;
        public int connection_urls_length;

        public List<String> getTags() {
            if (tags_length > 0) {
                return Arrays.asList(tags.getStringArray(0, tags_length));
            }
            return Collections.emptyList();
        }
        public List<URI> getConnectionUrls() {
            if (connection_urls_length > 0) {
                return Arrays.stream(connection_urls.getStringArray(0, connection_urls_length)).map(s -> {
                    try {
                        return new URI(s);
                    } catch (URISyntaxException e) {
                        e.printStackTrace();
                        return null;
                    }
                }).collect(Collectors.toList());
            }
            return Collections.emptyList();
        }
        public List<String> getCodecs() {
            if (codecs_length > 0) {
                return Arrays.asList(codecs.getStringArray(0, codecs_length));
            }
            return Collections.emptyList();
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList(
                    "id",
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
        void advertise_local_service(Pointer discoInstance, ServiceDescriptorInternal.ByValue sdi);

        void on_ready(Pointer m, Callback cb);

        ServiceDescriptorInternal.ByValue get_service_named(Pointer discoInstance);
        String[] get_service_names();
        ServiceDescriptorInternal.ByValue get_service_with_tags(Pointer discoInstance, Pointer tags, int tagCount);

        void destroy_descriptor(ServiceDescriptorInternal.ByReference ref);
        void shutdown(Pointer discoInstance);

        Pointer create();
    }

    public NativeDiscovery(String libName) throws IOException {
        discoLib = Native.loadLibrary(libName,
                DiscoveryLib.class);
        instance = discoLib.create();
    }

    @Override
    public List<String> getServiceNames() {
        return Arrays.asList(discoLib.get_service_names());
    }

    @Override
    public Optional<ServiceDescriptor> getServiceNamed(String s) {
        ServiceDescriptorInternal.ByValue disco = discoLib.get_service_named(instance);


        if (disco == null) return Optional.empty();
        System.out.println("TAGS = " + disco.tags_length);
        System.out.println("ID = " + disco.id);
        System.out.println("SVC = " + disco.identifier);
        ServiceDescriptor external = makeFrom(disco);
////        discoLib.destroy_descriptor(disco);
        return Optional.of(external);
    }

    private ServiceDescriptor makeFrom(ServiceDescriptorInternal internal) {
        List<InstanceDescriptor> instances = new ArrayList<>();

        System.out.println("Chasing the dragon");

        instances.add(new InstanceDescriptor(
                internal.id, internal.identifier, internal.getTags(), internal.getCodecs(), internal.getConnectionUrls(), Collections.emptyList()
        ));

        return new ServiceDescriptor(
                internal.identifier, internal.getTags(), internal.getCodecs(), Collections.emptyList(), instances);
    }

    @Override
    public Optional<ServiceDescriptor> getServiceWithTags(String... strings) {
        ServiceDescriptorInternal disco = discoLib.get_service_with_tags(instance, makeStringArray(strings), strings.length);
        if (disco == null) return Optional.empty();
        return Optional.of(makeFrom(disco));
    }

    @Override
    public void advertiseLocalService(InstanceDescriptor serviceDescriptor) {
        ServiceDescriptorInternal.ByValue sd = new ServiceDescriptorInternal.ByValue();
        sd.id = serviceDescriptor.getInstanceId();
        sd.identifier = serviceDescriptor.getIdentifier();

        sd.tags = makeStringArray(serviceDescriptor.getTags());
        sd.codecs = makeStringArray(serviceDescriptor.getCodecs());
        sd.connection_urls = makeStringArrayFromUrl(serviceDescriptor.getConnectionUrls());

        sd.tags_length = serviceDescriptor.getTags().size();
        sd.codecs_length = serviceDescriptor.getCodecs().size();
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
