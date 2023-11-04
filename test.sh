cargo build
cp ./target/debug/main ./dosbox
gcc test.c -o test
./dosbox -conf ./base/plutonia.conf -fullscreen -exit
