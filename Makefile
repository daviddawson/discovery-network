

lib:
	cargo build --release
	cp target/release/libmuon_discovery_net.so java/discovery-network/src/main/resources/linux-x86-64/


publish: version lib
	$(MAKE) -C java package

version:
ifndef VERSION
	$(error VERSION is undefined for Net Discovery Release)
endif
	$(MAKE) -C java version
	echo "done version"
	git commit -m "Update version to $(VERSION )while publishing"
	git push origin

test:
	cargo test-xunit
