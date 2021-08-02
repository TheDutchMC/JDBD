.PHONY: mysql build publish test
all: build

CARGO=cargo
GRADLE=./gradlew

mysql: libs/jdbd_mysql/target/x86_64-unknown-linux-gnu/release/libjdbd_mysql.so libs/jdbd_mysql/target/x86_64-pc-windows-gnu/release/libjdbd_mysql.dll

MYSQL_SRC := $(shell find libs/jdbd_mysql -type f -name '*.rs')
libs/jdbd_mysql/target/x86_64-unknown-linux-gnu/release/libjdbd_mysql.so: ${MYSQL_SRC}
	cd libs/jdbd_mysql; \
		${CARGO} build --target x86_64-unknown-linux-gnu --release

libs/jdbd_mysql/target/x86_64-pc-windows-gnu/release/libjdbd_mysql.dll: ${MYSQL_SRC}
	cd libs/jdbd_mysql; \
		${CARGO} build --target x86_64-pc-windows-gnu --release
	cp libs/jdbd_mysql/target/x86_64-pc-windows-gnu/release/jdbd_mysql.dll libs/jdbd_mysql/target/x86_64-pc-windows-gnu/release/libjdbd_mysql.dll

JAVA_SRC := $(shell find src/ -type f -name '*.java')
build: mysql ${JAVA_SRC}
	${GRADLE} build

publish: mysql ${JAVA_SRC}
	${GRADLE} publish

publish-local: mysql ${JAVA_SRC}
	${GRADLE} publishtomavenlocal

test:
	${GRADLE} test