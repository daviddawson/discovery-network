package io.muoncore.discovery.jna;

import io.muoncore.Muon;
import io.muoncore.MuonBuilder;
import io.muoncore.ServiceDescriptor;
import io.muoncore.config.AutoConfiguration;
import io.muoncore.config.MuonConfigBuilder;
import io.muoncore.protocol.reactivestream.server.PublisherLookup;
import org.scijava.nativelib.NativeLibraryUtil;
import org.scijava.nativelib.NativeLoader;
import reactor.core.processor.CancelException;
import reactor.rx.broadcast.Broadcaster;

import java.io.IOException;
import java.util.Collections;

import static io.muoncore.protocol.requestresponse.server.HandlerPredicates.path;
import static java.util.Collections.emptyList;

public class HelloJvm {

    public static void main(String[] args) throws IOException, InterruptedException {
//        NativeLibraryUtil.loadNativeLibrary(HelloJvm.class, "muon_discovery_net");
//        NativeLoader.loadLibrary("muon_discovery_net");
//
        NativeDiscovery disco = new NativeDiscovery("muon_discovery_net");
//
//        disco.onReady(() -> {
//            System.out.println("WOOT");
//        });
//
//        disco.advertiseLocalService(new ServiceDescriptor("wibble" + System.currentTimeMillis(), emptyList(), emptyList(), emptyList(), emptyList()));
//
//        //TODO, somehow integrate https://github.com/scijava/native-lib-loader
//
//        Thread.sleep(40000);
//        disco.shutdown();
//        DiscoInstance muon = getMuon();

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
