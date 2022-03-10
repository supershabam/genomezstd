# genomezstd

## goals

* determine compression available from re-encoding fasta files to binary  
* determine compression available from zstd  

## experiments
### naive text zstd compression

Inputs:  
* https://www.ncbi.nlm.nih.gov/assembly/GCA_000001405.29  

To build a dictionary, many training examples must be provided which are at max ~130kb each.
So, we need to split the 3.1G .fna file.
However, the `split` command will not split into too many files, so we have to take a 2-tiered approach.

```
mkdir temp && cd temp
split -b10000k ../GCA_000001405.29_GRCh38.p14_genomic.fna
mkdir ../training && cd ../training
split -b100k ../temp/xaa
cd ..
zstd --train training/*
zstd -D dictionary GCA_000001405.29_GRCh38.p14_genomic.fna -o genomic.fna.zstd
```

result
```
-rw-r--r--   1 supershabam  staff   991M Mar 10 10:20 genomic.fna.zstd
-rw-r--r--    1 supershabam  staff   3.1G Mar 10 10:20 GCA_000001405.29_GRCh38.p14_genomic.fna
```

_Conclusion_  

Naive zstd usage yields a compression ratio of 3.1.  
The fna file (and zstd dictionary) uses capital and lowercase nucleotide letters.  

### fna to binary representation

Inputs:  
* https://www.ncbi.nlm.nih.gov/assembly/GCA_000001405.29  

Alphabet:  
```
| letter | binary |
| --- | --- |
| a | b0000 |
| c | b0001 |
| g | b0010 |
| t | b0011 |
| n | b0100 |
| m | b0101 |
| r | b0110 |
| y | b0111 |
| w | b1000 |
| k | b1001 |
| b | b1010 |
| s | b1011 |
| fill | b1111 |
```