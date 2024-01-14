HDC-rs
==============

Rust version of the language recognition algorithm in the Py/Matlab/Verilog directories above.
Uses binary rather than bi-polar vectors [4].

References:


- [1] ["What We Mean When We Say 'What’s the Dollar of Mexico?': Prototypes and Mapping in Concept Space" by Pentti Kanerva, Quantum Informatics for Cognitive, Social, and Semantic Processes: Papers from the AAAI Fall Symposium (FS-10-08), 2010](https://aaai.org/papers/02243-2243-what-we-mean-when-we-say-whats-the-dollar-of-mexico-prototypes-and-mapping-in-concept-space/) 
- [2] "Language Geometry using Random Indexing" Aditya Joshi1, Johan T. Halseth, and Pentti Kanerva, 2016
- [3] "A Robust and Energy-Efficient Classifier Using Brain-Inspired Hyperdimensional Computing" Abbas Rahimi, Pentti Kanerva, Jan M. Rabaey, 2016
- [4] ["Hyperdimensional Computing: An Algebra for Computing with Vectors", Pentti Kanerva, 2022](https://redwood.berkeley.edu/wp-content/uploads/2022/05/kanerva2022hdmss.pdf)


Run
-----

```
% cd HDC-Language-Recognition/Rs
% cargo run --release
```

Benchmark
---------

Below we will benchmark the time it takes to train and test a model.
There are 21 test languages (1k sentences per language) and 22 training languages (21+afr, 10k sentences each). All times are in seconds (wall clock) on a Macbook Air M1 (2020, 8 cores). 

Number of bits is rounded up to fill a whole even number of usize elements.

```
% time cargo run --release
```

### Accuracy 
| ngram | hdv bits| Accuracy    | Time        |  
| ----: | --:     | ---------:  | ----------: | 
| 1     |   10112 | 73.0%       | 114         | 
| 2     |   10112 | 93.1%       | 111         | 
| 3     |   10112 | 96.6%       | 110         | 
| 4     |   10112 | 96.8%       | 110         | 
| 5     |   10112 | 95.0%       | 108         | 
| 6     |   10112 | 88.4%       | 111         | 
| 2     |  100096 | 93.2%       | 1194        | 
| 3     |  100096 | 96.9%       | 1083        | 
| 4     |  100096 | 98.0%       | 1087        | 
| 5     |  100096 | 98.1%       | 1084        | 
| 6     |  100096 | 97.7%       | 1076        | 
| 4     | 1000064 | 98.1%       | 20139       | 
| 5     | 1000064 |   . %       |             | 
| 6     | 1000064 | 98.6%       | 28830       | 

