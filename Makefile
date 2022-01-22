.PHONY: lib build publish test
all: build

CARGO=cargo
GRADLE=./gradlew

lib: lib/target/x86_64-unknown-linux-gnu/release/libjdbd.so lib/target/x86_64-pc-windows-gnu/release/libjdbd.dll

RUST_SRC := $(shell find lib/ -type f -name '*.rs')

lib/target/x86_64-unknown-linux-gnu/release/libjdbd.so: ${RUST_SRC}
	cd lib/; \
		${CARGO} build --target x86_64-unknown-linux-gnu --release

lib/target/x86_64-pc-windows-gnu/release/libjdbd.dll: ${RUST_SRC}
	cd lib/; \
		${CARGO} build --target x86_64-pc-windows-gnu --release
	cp lib/target/x86_64-pc-windows-gnu/release/jdbd.dll lib/target/x86_64-pc-windows-gnu/release/libjdbd.dll

JAVA_SRC := $(shell find src/ -type f -name '*.java')
build: lib
	${GRADLE} build

publish: lib
	${GRADLE} publish

publish-local: lib
	${GRADLE} publishtomavenlocal

test:
	${GRADLE} test