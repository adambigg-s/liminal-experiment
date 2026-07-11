@echo on

cargo +nightly build --release

copy target\release\backrooms_liminal.exe .
