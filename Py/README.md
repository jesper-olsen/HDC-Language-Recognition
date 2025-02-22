HDC-py
==============

Python version of the Matlab/Verilog language recogniton algorithm above.
See Rs directory for a rust version that uses binary rather than bi-polar vectors.

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

### Accuracy 

| ngram | dim    | Accuracy    | Time        |  
| ----: | --:    | ---------:  | ----------: | 
| 2     | 100    | 87.1%       | 573         | 
| 3     | 100    | 77.1%       | 542         | 
| 4     | 100    | 53.9%       | 617         | 
| 2     | 1000   | 93.6%       | 906         | 
| 3     | 1000   | 95.9%       | 922         | 
| 4     | 1000   | 93.8%       | 942         | 
| 2     | 10000  | 94.1%       | 4313        | 
| 3     | 10000  | 97.1%       | 4331        | 
| 4     | 10000  | 98.0%       | 4353        | 
| 5     | 10000  | 97.3%       | 4451        | 
| 5     | 100000 | 98.5%       | 39644       | 

#
### Confusion Matrix
 
Confusion Matrix for the 4-gram, 10000-dim case

||afr|bul|ces|dan|deu|ell|eng|est|fin|fra|hun|ita|lav|lit|nld|pol|por|ron|slk|slv|spa|swe|
|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|-:|
|afr|0|0|0|0|9|0|1|0|0|0|0|0|0|0|19|0|0|0|0|0|0|1|
|bul|0|987|2|0|0|5|0|0|1|2|0|0|0|0|0|4|1|0|1|6|0|0|
|ces|0|6|944|1|0|0|1|1|0|0|0|0|1|2|0|1|0|0|51|8|0|0|
|dan|0|0|0|979|1|0|1|1|0|0|0|0|0|0|1|1|0|0|0|0|1|8|
|deu|0|1|0|0|982|1|1|1|0|0|0|0|0|0|4|0|1|0|1|0|0|3|
|ell|0|1|1|0|0|990|1|0|0|3|0|0|1|0|0|0|0|0|1|0|0|0|
|eng|0|0|0|2|0|1|976|1|0|0|0|0|0|0|0|1|2|0|0|0|0|1|
|est|0|0|0|0|0|1|0|983|2|0|0|0|0|0|1|1|1|0|0|0|1|0|
|fin|0|0|1|0|0|0|0|6|993|0|0|0|0|0|1|0|0|0|0|0|0|1|
|fra|0|0|1|2|1|0|12|4|0|985|0|0|1|0|0|1|1|2|0|1|0|0|
|hun|0|0|1|1|0|0|0|0|0|0|997|0|0|0|0|0|0|0|0|0|0|0|
|ita|0|0|1|0|0|1|0|0|0|0|2|990|0|1|0|0|2|0|0|1|6|1|
|lav|0|0|1|0|1|0|0|1|3|0|0|0|979|2|0|0|0|0|0|0|3|0|
|lit|0|0|0|0|0|0|1|0|1|0|0|0|15|995|0|0|0|0|1|1|2|0|
|nld|0|2|0|4|1|0|0|1|0|1|0|0|0|0|973|0|0|0|1|1|1|2|
|pol|0|1|0|0|0|0|0|0|0|0|0|1|1|0|0|980|0|0|13|0|0|0|
|por|0|0|0|0|0|0|1|0|0|4|0|5|1|0|1|0|989|2|0|0|36|0|
|ron|0|0|1|0|0|1|2|0|0|0|0|2|0|0|0|1|0|993|1|0|0|0|
|slk|0|2|32|0|0|0|1|1|0|1|1|0|1|0|0|7|0|1|910|2|0|0|
|slv|0|0|14|0|1|0|0|0|0|0|0|0|0|0|0|3|0|0|20|979|0|0|
|spa|0|0|0|0|0|0|0|0|0|4|0|2|0|0|0|0|3|2|0|1|950|0|
|swe|0|0|1|11|4|0|2|0|0|0|0|0|0|0|0|0|0|0|0|0|0|983|
