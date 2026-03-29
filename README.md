# injectdb

A Rust library with Python bindings for detecting SQL injection patterns in input strings. Built for speed and designed especially for AI agents that process or generate SQL queries.

## Building from Source

Requires [Rust](https://rustup.rs/), [Maturin](https://github.com/PyO3/maturin) & [UV](https://docs.astral.sh/uv/).

```bash
git clone https://github.com/kanishknavale/injectdb
cd injectdb
pip install maturin
make release
```

## Usage

- Python

    ```python
    from injectdb import audit_query

    audit_query("SELECT * FROM users WHERE id = 1")
    # False

    audit_query("SELECT * FROM users WHERE id = 1 OR 1=1 --")
    # True

    audit_query("'; DROP TABLE users; --")
    # True
    ```

- Rust

    ```rust
    use injectdb::audit_query;

    fn main() {
        let safe = audit_query("SELECT * FROM users WHERE id = 1");
        println!("{}", safe); // false

        let malicious = audit_query("SELECT * FROM users WHERE id = 1 OR 1=1 --");
        println!("{}", malicious); // true
    }
    ```

## Testing

- Sanity Tests

    ```bash
    make test
    ```

- Dataset evaluation (requires dataset)

    Download the [RbSQLi dataset](https://data.mendeley.com/datasets/xz4d5zj5yw/3) and place it at `tests/data/wild.csv`, then run:

    ```bash
    make test-wild
    ```

## Benchmarks

Evaluated against the [RbSQLi dataset](https://data.mendeley.com/datasets/xz4d5zj5yw/3) containing 10,304,026 labeled SQL queries (2,813,146 malicious, 7,490,880 benign).

- Confusion Matrix

    Actual \ Predicted |   Malicious   |     Benign    |
    -------------------|---------------|---------------|
    Actual Malicious   |   2,813,146   |           0   |
    Actual Benign      |           0   |   7,490,880   |

## License

[MIT](LICENSE)
