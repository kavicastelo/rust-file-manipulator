@echo off
cargo build --release
mkdir package
copy target\release\file_manipulator.exe package\
copy README.md package\
cd package
zip -r file_manipulator.zip .
cd ..
rmdir /s /q package
