var ffi = require('ffi');
var ref = require('ref');
var Struct = require('ref-struct');
var RefArray = require('ref-struct');

var discovery = ref.types.void;
var discoveryPtr = ref.refType(discovery);

var StringArray = RefArray("string")

var ServiceDescriptor = Struct({
    identifier: 'string'
    // tags: ref.refType,
    // codecs:  1),
    // connection_urls: RefArray('float', 1)
});
var ServiceDescriptorPtr = ref.refType(ServiceDescriptor);

var lib = ffi.Library('../target/release/libmuon_discovery_net', {
  create: [discoveryPtr, ['string']],
  advertise_local_service: [discovery, [ServiceDescriptorPtr, 'pointer']]
});

var disco = lib.create("WOOT!!! YO DUD");

var service = new ServiceDescriptor();
service.identifier = "SImples"

lib.advertise_local_service(disco, service)

setTimeout(function() {
  console.log("HELLO!")
}, 10000)
