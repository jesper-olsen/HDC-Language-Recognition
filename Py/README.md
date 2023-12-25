HDC-py
==============

Python version of the Matlab/Verilog language recogniton algorithm above.

Run
-----

```
% cd HDC-Language-Recognition/Py
% python hdc.py -h
usage: hdc.py [-h] [-n NGRAM] [-d DIM]

train and evaluate HD language identification model.

options:
  -h, --help            show this help message and exit
  -n NGRAM, --ngram NGRAM
  -d DIM, --dim DIM
```

Benchmark
---------

Below we will benchmark the time it takes to train and test a model.
There are 21 test languages (1k sentences per language) and 22 training languages (21+afr, 10k sentences each). All times are in seconds on a Macbook Air M1 (2020, 8 cores). 

```
% time python hdc.py -n 2 -d 100
```

### Sequential 

| ngram | dim   | Accuracy    | Time (real) |  
| ----: | --:   | ---------:  | ----------: | 
| 2     | 100   | 87.0%       | 450         | 
| 2     | 1000  | 93.7%       | 755         | 
| 2     | 10000 | 94.2%       | 6631        | 
| 3     | 10000 | 97.3%       | 4189        | 
| 4     | 10000 | 98.0%       | 4388        | 
