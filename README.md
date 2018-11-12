# TBE

Truncated Binary Encoding

https://en.wikipedia.org/wiki/Truncated_binary_encoding

```rust
k = floorLog2(n);
u = 2 * exp2(k) - n
```

## Writing (MSB first)

if `x < u` write `k` bits of `x` else add `u` to `x` and write `k + 1` bits.

## Reading (MSB first)

1. read `k` bits as `x`.
2. if `u` <= `x` then read an additional bit and add it to `x` as a low bit and substract `u`.

## Examples

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

BE|TBE| |
--|---|-|
00|  0| |
01| 10|u|
10| 11| |

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

  BE| TBE|
----|----|
0000| 000|
0001| 001|
0010| 010|
0011| 011|
0100| 100|
0101| 101|
0110|1100|
0111|1101|
1000|1110|
1001|1111|
