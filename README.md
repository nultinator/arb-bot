# arb-bot
<h1>Still Under Heavy Devlopment</h1>
<h2>Getting Started</h2>
<p>
First, you will need to have <strong>rust</strong> and <strong>cargo</strong> installed. Afterward, go ahead and clone this repo.

```
git clone https://github.com/nultinator/arb-bot
```

<strong>MAKE SURE YOU HAVE A BALANCE</strong>...example:<br>You are buying [some random coin] with BTC, you need to make sure you have enough BTC!!!
</p>

<h2>Running the Bot</h2>

```
cd arb-bot
```

```
cargo run
```

<p>
Follow the prompts and walk feel free to walk away!
</p>

![Screenshot from 2023-03-14 11-31-07](https://user-images.githubusercontent.com/72562693/225053476-13afa5ba-25d0-4ed5-bb7f-bca631397c9d.png)

<h2>Strategies</h2>
  <h3>Scheduled Arb</h3>
  <ol>
    <li>Follow the prompts to enter your base coin and trading pairs
    <li>Set a target spread and a schedule
    <li>Runs an arbitrage function across all pairs at the schedule interval of your choosing
  </ol>
  <h3>Listen and React</h3>
  <ol>
    <li>Choose a trading pair to watch
    <li>Stream the live orderbook
  </ol>
  <h3>Triangle Arb (Needs renaming: Takes an infinite amount of pairs)</h3>
  <ol>
    <li>Follow the prompts to enter a base coin and trading pairs
    <li>Arbitrage across all of your pairs every time an order is posted in pair number 1
  </ol>
  <h3>DCA (Dollar Cost Averaging)</h3>
  <ol>
    <li>Select a coin to buy (example: BTC) and a coin to buy it with (example: USDT)
    <li>Choose an amount to buy and a schedule
    <li>Buy that amount at scheduled intervals
  </ol>

<h1>USE AT YOUR OWN RISK</h1>

