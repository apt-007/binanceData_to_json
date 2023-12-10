## 币安csv文件转换json格式工具
- 为了适配barter-rs[https://docs.rs/barter/latest/barter]回测数据结构的小工具


## 币安历史数据介绍
https://github.com/binance/binance-public-data

## 使用说明
- binance_historical_data[https://github.com/stas-prokopiev/binance_historical_data]
```python
pip install binance_historical_data
```

- cmd/shell命令进入python

## 现货数据
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
## 期货数据-U本位
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
## 期货数据-币本位
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
- 修改base_path 为你的下载数据列表
- 修改output_path 为你的保存文件夹
- 修改main.rs 的time函数为你需要把CSV转换为json的k线周期 例如 1m,1h,4h