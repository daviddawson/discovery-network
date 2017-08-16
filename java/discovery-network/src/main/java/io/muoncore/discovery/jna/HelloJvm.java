package io.muoncore.discovery.jna;

import io.muoncore.InstanceDescriptor;

import java.io.IOException;
import java.util.Arrays;
import java.util.UUID;

import static java.util.Collections.emptyList;

public class HelloJvm {

    public static void main(String[] args) throws IOException, InterruptedException {
//        NativeLibraryUtil.loadNativeLibrary(HelloJvm.class, "muon_discovery_net");
//        NativeLoader.loadLibrary("muon_discovery_net");
//

        /**
         * TODO
         *
         * get_service_names
         * get_service_named
         *get_service_with_tags
         *
         * add a logging method to allow using host language logging.
         * info
         * debug
         *
         */


        NativeDiscovery disco = new NativeDiscovery("muon_discovery_net");


        disco.onReady(() -> {
            System.out.println("WOOT");
        });

        disco.advertiseLocalService(
                new InstanceDescriptor(UUID.randomUUID().toString(), "wibble" + System.currentTimeMillis(),
                        Arrays.asList("hello", "world", "simple"),
                        emptyList(), emptyList(), emptyList()));

        Thread.sleep(1000);

        disco.getServiceNamed("wibble").ifPresent(serviceDescriptor -> {
            System.out.println("svc = " + serviceDescriptor.getIdentifier());
        });

//        System.out.println("svc = " + myService.getIdentifier());
//
        Thread.sleep(6000);
        disco.shutdown();

//        muon.getDiscovery().blockUntilReady();

//        muon.handleRequest(path("/"), requestWrapper -> requestWrapper.ok("Hello World"));

//        tickTock(muon);
    }

//    private static void tickTock(DiscoInstance muon) {
//        Broadcaster b = Broadcaster.create();
//
//        muon.publishSource("/ticktock", PublisherLookup.PublisherType.HOT, b);
//
//        Thread t = new Thread(() -> {
//            while(true) {
//                try {
//                    try {
//                        b.accept(Collections.singletonMap("time", "hello " + System.currentTimeMillis()));
//                    } catch (CancelException e) {}
//                    Thread.sleep(2000);
//                } catch (InterruptedException e) {
//                    e.printStackTrace();
//                }
//            }
//        });
//
//        t.start();
//    }


//    private static DiscoInstance getMuon() {
//        AutoConfiguration config = MuonConfigBuilder
//                .withServiceIdentifier("hello-jvm")
//                .addWriter( autoConfiguration -> {
//        if (System.getenv().containsKey("MUON_URL")) {
//            autoConfiguration.getProperties().put("amqp.transport.url", System.getenv().get("MUON_URL"));
//            autoConfiguration.getProperties().put("amqp.discovery.url", System.getenv().get("MUON_URL"));
//        }
//        }).build();
//
//        return MuonBuilder.withConfig(config).build();
//    }
}
