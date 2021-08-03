.PHONY: lib build publish test
all: build

CARGO=cargo
GRADLE=./gradlew

lib: lib/target/x86_64-unknown-linux-gnu/release/libjdbd.so libs/target/x86_64-pc-windows-gnu/release/libjdbd.dll

MYSQL_SRC := $(shell find lib/ -type f -name '*.rs')
lib/target/x86_64-unknown-linux-gnu/release/libjdbd.so: ${MYSQL_SRC}
	cd lib/; \
		${CARGO} build --target x86_64-unknown-linux-gnu --release

lib/target/x86_64-pc-windows-gnu/release/libjdbd_mysql.dll: ${MYSQL_SRC}
	cd lib/; \
		${CARGO} build --target x86_64-pc-windows-gnu --release
	cp lib/target/x86_64-pc-windows-gnu/release/jdbd.dll lib/target/x86_64-pc-windows-gnu/release/libjdbd.dll

JAVA_SRC := $(shell find src/ -type f -name '*.java')
build: mysql ${JAVA_SRC}
	${GRADLE} build

publish: mysql ${JAVA_SRC}
	${GRADLE} publish

publish-local: mysql ${JAVA_SRC}
	${GRADLE} publishtomavenlocal

test:
	${GRADLE} test