# 1. Getting ISA-L
```
mkdir wks; cd wks
git clone https://github.com/intel/isa-l
cd isa-l
```

# 2. Compiling Libraries

```
./autogen.sh
./configure
make
```
To do benchmarks, we do not need to install ISA-L by `make install`.

# 3. Benchmark
To run a benchmark set provided by ISA-L,
```
make perf
./erasure_code/erasure_code_perf
```

To run a benchmark program, which is based on Intel's benchmark code and  used in the paper,
```
gcc -O3 -DDATA_BLOCK=10 -DPARITY_BLOCK=4 -DDATA_SIZE=10000000 my_perf.c -Iinclude -lisal -o my_perf
./my_perf
```
By changing the parameters -DDATA_BLOCK=d and -DPARITY_BLOCK=p, we can run **RS(d, p)**.
