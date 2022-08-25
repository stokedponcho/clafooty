# clafooty

Command line and footy!

## How to install

```bash
make install
```

If it is the first time installing, the [https://www.footballdata.org] API token needs to be added to `.cargo/config.toml`:

```bash
mkdir -p .cargo && printf %"s\n" "[env]" "FOOTBALLDATA_API_AUTH_TOKEN = \"<YOUR FOOTBALL DATA API KEY>\"" >> .cargo/config.toml
```

