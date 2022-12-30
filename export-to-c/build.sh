cargo build
gcc -static main.c -L $PWD/../target/debug -l primes -o main
echo "Build finished"
