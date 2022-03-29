# [bytify] Make human-readable representation of bytes

This program uses IEC-compliant units, see the table below:

| **UNIT NAME** | **VALUE (in bytes)** | **SHORTHAND** |
|:-------------:|:--------------------:|:-------------:|
| kibibyte      | 1024^1               | KiB           |
| mebibyte      | 1024^2               | MiB           |
| gibibyte      | 1024^3               | GiB           |
| tebibyte      | 1024^4               | TiB           |
| pebibyte      | 1024^5               | PiB           |
| exbibyte      | 1024^6               | EiB           |
| zebibyte      | 1024^7               | ZiB           |
| yobibyte      | 1024^8               | YiB           |

## [🛠️] Build

```
git clone git@github.com:yohgen/bytify.git   # clone repository
cd ./bytify                                  # change directory
cargo build --release                        # compile with cargo
```

## [🏃] Run

```
# long argument notation
./target/release/bytify --bytes=6666 --precision=4
> 6.5098 KiB

# short argument notation
./target/release/bytify -b 987654321 -p 0
> 942 MiB

# the default precision is 2
./target/release/bytify -b 99999999999999999999999
> 84.70 ZiB

# handles values greater than the currently standardized yobibyte
./target/release/bytify -b 100000000000000000000000000000
> +1024.00 YiB
```
