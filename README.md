# TBE

Truncated Binary Encoding

https://en.wikipedia.org/wiki/Truncated_binary_encoding

```rust
k = floor_log2(n);
u = 2 * pow2(k) - n
```

## Writing

if `x < u` write `k` least significant bits of `x` else add `u` to `x` and write `k` most significant bits of `x` and then write a least significant bit of `x`.

## Reading

1. read `k` bits as `x`.
2. if `u` <= `x` then read an additional bit and add it to `x` as a least significant bit and substract `u`.

## Examples (MSB first)

### n = 2

```rust
k = 1;
u = 4 - 2 = 2;
```

BE|TBE| |
--|---|-|
 0|  0| |
 1|  1| |
 n|   |u|

### n = 3

```rust
k = 2;
u = 1
```

BE|TBE(MSB)|TBE(LSB)| |
--|--------|--------|-|
00|      0X|      0X| |
01|      10|      10|u|
10|      11|      11| |

### n = 4

```
k = 2
u = 8 - 4 = 4
```

BE|TBE| |
--|---|-|
00| 00| |
01| 01| |
10| 10| |
11| 11| |
 n|   |u|

### n = 5

```
k = 2;
u = 3;
```

 BE|TBE| |
---|---|-|
000| 00| |
001| 01| |
010| 10| |
011|110|u|
100|111| |

### n = 6

```
k = 2;
u = 2;
```

 BE|TBE|
---|---|
000| 00|
001| 01|
010|100|
011|101|
100|110|
101|111|

### n = 7

 BE|TBE|
---|---|
000| 00|
001|010|
010|011|
011|100|
100|101|
101|110|
110|111|

### n = 10

```
k = 3;
u = 6;
```

  BE|TBE(MSB)|TBE(LSB)|
----|--------|--------|
0000|    000X|    000X|
0001|    001X|    100X|
0010|    010X|    010X|
0011|    011X|    110X|
0100|    100X|    001X|
0101|    101X|    101X|
0110|    1100|    0110|
0111|    1101|    0111|
1000|    1110|    1110|
1001|    1111|    1111|
