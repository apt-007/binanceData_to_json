## Binance CSV to JSON Format Tool
- A small tool designed to adapt to the data structure of [barter-rs](https://docs.rs/barter/latest/barter) for backtesting.

## Binance Historical Data Introduction
[https://github.com/binance/binance-public-data](https://github.com/binance/binance-public-data)

## Instructions
- [binance_historical_data](https://github.com/stas-prokopiev/binance_historical_data)
```python
pip install binance_historical_data
```

- Run the following command in cmd/shell to enter the Python environment.

## Spot Data
```python
from binance_historical_data import BinanceDataDumper

data_dumper = BinanceDataDumper(
    path_dir_where_to_dump="/home/user/.",
    asset_class="spot",  # spot, um, cm
    data_type="klines",  # aggTrades, klines, trades
    data_frequency="1m",
)
data_dumper.dump_data(
    tickers=None,
    date_start=None,
    date_end=None,
    is_to_update_existing=False,
    tickers_to_exclude=["UST"],
)

```
## Futures Data - USDT-Margined
```python
data_dumper = BinanceDataDumper(
    path_dir_where_to_dump="/home/user/.",
    asset_class="um",  # spot, um, cm
    data_type="klines",  # aggTrades, klines, trades
    data_frequency="1m",
)
data_dumper.dump_data(
    tickers=None,
    date_start=None,
    date_end=None,
    is_to_update_existing=False,
    tickers_to_exclude=["UST"],
)

```
## Futures Data - Coin-Margined
```python
data_dumper = BinanceDataDumper(
    path_dir_where_to_dump="/home/user/.",
    asset_class="cm",  # spot, um, cm
    data_type="klines",  # aggTrades, klines, trades
    data_frequency="1m",
)
data_dumper.dump_data(
    tickers=None,
    date_start=None,
    date_end=None,
    is_to_update_existing=False,
    tickers_to_exclude=["UST"],
)
```
- Modify `base_path` to your downloaded data list.
- Modify `output_path` to your desired save folder.
- Modify the `time` function in `main.rs` to your desired candlestick period for converting CSV to JSON, for example, 1m, 1h, 4h.