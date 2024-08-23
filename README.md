Toy SQL parser in Rust using logos and lalrpop.

```sh
$ cargo run --quiet
>> SELECT * FROM abc
Query(
    SELECT(
        [
            Star,
        ],
    ),
    FROM(
        Identifier(
            "abc",
        ),
    ),
    None,
)
>> SELECT abc FROM  xyz
Query(
    SELECT(
        [
            Identifier(
                Identifier(
                    "abc",
                ),
            ),
        ],
    ),
    FROM(
        Identifier(
            "xyz",
        ),
    ),
    None,
)
```