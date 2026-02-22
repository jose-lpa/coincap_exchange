# CoinCap Exchange

A tiny CLI app that reaches [CoinCap API](https://pro.coincap.io/api-docs/) to extract 
cryptocurrencies exchange data in real time, dumping the results into a CSV file.

Simply a learning exercise for myself to turn a DuckDB script from 
[Joseph Machado's](https://github.com/josephmachado/cost_effective_data_pipelines) Data Engineering
exercises from Python code into a Rust CLI tool.

## Usage

> [!IMPORTANT]
> In order to use CoinCap API you need an API token. The free tier is more than enough
> to run the example in this repo and you can sign up for it and obtain a token by
> following the instructions in [CoinCap API documentation](https://pro.coincap.io/api-docs/).

Simply run the application from your terminal, passing the name of the file where you want to store
the results.

You can pass the CoinCap API token in different ways. Choose the one that fits your needs:

1. Pass it directly in the command line arguments:
   ```shell
   coincap_exchange --file results.csv --api-key <COINCAP_API_KEY>
   ```
2. Declare it as an environment variable and simply call the app. E.g. in Bash shell:
  ```bash
   export COINCAP_API_KEY=<COINCAP_API_KEY>
  ```
3. Store it in a `.env` file in the same directory where you run the CLI, which is automatically
   picked up by the app:
   ```
   COINCAP_API_KEY=<COINCAP_API_KEY>
   ```
   ```shell
   coincap_exchange --file results.csv
   ```