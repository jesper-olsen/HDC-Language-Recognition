HDC-rs
==============

Rust version of the language recognition algorithm in the Py/Matlab/Verilog directories above.
Uses binary rather than bi-polar vectors.

Run
-----

```
% cd HDC-Language-Recognition/Rs
% cargo run --release
```

Benchmark
---------

Below we will benchmark the time it takes to train and test a model.
There are 21 test languages (1k sentences per language) and 22 training languages (21+afr, 10k sentences each). All times are in seconds on a Macbook Air M1 (2020, 8 cores). 

Number of bits is rounded up to fill a whole number of usize elements.

```
% time cargo run --release
```

### Accuracy 
| ngram | dim    | Accuracy    | Time        |  
| ----: | --:    | ---------:  | ----------: | 
| 1     |  10112 | 73.0%       | 114         | 
| 2     |  10112 | 93.1%       | 111         | 
| 3     |  10112 | 96.6%       | 110         | 
| 4     |  10112 | 96.8%       | 110         | 
| 5     |  10112 | 95.0%       | 108         | 
| 6     |  10112 | 88.4%       | 111         | 
| 2     | 100096 | 93.2%       | 1194        | 
| 3     | 100096 | 96.9%       | 1083        | 
| 4     | 100096 | 98.0%       | 1087        | 
| 5     | 100096 | 98.1%       | 1084        | 
| 6     | 100096 | 97.7%       | 1076        | 

