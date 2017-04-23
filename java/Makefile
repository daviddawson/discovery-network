
all: package

package: clean
	pwd
	cp ../target/release/libmuon_discovery_net.so discovery-network/src/main/resources/linux-x86-64/
	./gradlew artifactoryPublish

clean:
	./gradlew clean

test:
	./gradlew test

version:
	echo version=$(VERSION)>gradle.properties
	echo group=io.muoncore>>gradle.properties
	echo rootOnly=true>>gradle.properties
	echo exclude=>>gradle.properties
	git add gradle.properties
