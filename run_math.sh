echo "" | rustc --print native-static-libs --crate-type staticlib rmath.rs
gcc example.c -o example -L. -lrmath -lm -lpthread -ldl
./example
