

lib:
	cargo build --release


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
