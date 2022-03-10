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
-rw-r--r--@  1 supershabam  staff   928M Feb 23 18:05 genome_assemblies_genome_fasta.tar
-rw-r--r--   1 supershabam  staff   991M Mar 10 10:20 genomic.fna.zstd
-rw-r--r--    1 supershabam  staff   3.1G Mar 10 10:20 GCA_000001405.29_GRCh38.p14_genomic.fna
```

_Conclusion_  

* Naive zstd usage yields a compression ratio of 3.1.  
* The fna file (and zstd dictionary) uses capital and lowercase nucleotide letters.  
* *worse* compression than gzip.  

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

creating the binary version of the file

```
cargo run --bin encode > files/GCA_000001405.29_GRCh38.p14_genomic.fna.bin

-rw-r--r--    1 supershabam  staff   3.1G Mar 10 10:20 GCA_000001405.29_GRCh38.p14_genomic.fna
-rw-r--r--    1 supershabam  staff   1.5G Mar 10 11:18 GCA_000001405.29_GRCh38.p14_genomic.fna.bin
```

_conclusion_  

* expected compression ratio of 2 achieved (4 bit letters instead of 8 bit letters)  

### naive zstd compression of binary

```
rm -fr temp training
mkdir temp && cd temp
split -b10000k ../files/GCA_000001405.29_GRCh38.p14_genomic.fna.bin
mkdir ../training && cd ../training
split -b100k ../temp/xaa
cd ..
zstd --train training/*
zstd -D dictionary files/GCA_000001405.29_GRCh38.p14_genomic.fna.bin -o genomic.fna.bin.zstd
```

```
-rw-r--r--    1 supershabam  staff   3.1G Mar 10 10:20 GCA_000001405.29_GRCh38.p14_genomic.fna
-rw-r--r--    1 supershabam  staff   729M Mar 10 11:18 genomic.fna.bin.zstd
-rw-r--r--    1 supershabam  staff   991M Mar 10 10:20 genomic.fna.zstd
-rw-r--r--@  1 supershabam  staff   928M Feb 23 18:05 genome_assemblies_genome_fasta.tar
```

_conclusion_

* Naive zstd usage yields a compression ratio of 4.3.  
* Improvement upon text compression is merely compression ratio of 1.4.  
* Zstd dictionary created with naive sample set (arbitrarily chosen subsegment)  

## next steps

From an art project visualizing the human genome subsequence frequencies, it's clear that there are more dominant larger subsequences.  

It may be possible to build a zstd dictionary using this subsequence frequency analysis datastructure.  

Given that zstd is indeed able to produce better than gzip compression on the binary format, it's clear that there is room for improvement.  

However, <1GB for a compressed file is really not that large and storage costs are always trending downwards.  

![_image: subsequence frequency_](FJ9Tg-UH_400x400.jpg)
