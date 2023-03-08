# arb-bot
<h1>USE AT YOUR OWN RISK</h1>
<h2>Getting Started</h2>
<p>
First, you will need to have <strong>rust</strong> and <strong>cargo</strong> installed. Afterward, go ahead and clone this repo.

```
git clone https://github.com/nultinator/arb-bot
```

Next, in a code editor of your choice, change the following lines of <strong>"binance_us_api.rs"</strong>
![image](https://user-images.githubusercontent.com/72562693/223858190-92aaff79-1846-4054-8966-c9b5c3026c4c.png)

<p>
By default, the main is set to run for ADA and a set of pairs it trades with. If you wish to change these, at the moment, you need to open up <strong>"main.rs"</strong> and replace <strong><i>ADA</i></strong> with a coin of your choice.
You will then need to replace the list of trading pairs with other pairs for your new coin.
Afterward, you need to replace the spread variable, <strong><i>1.05</i></strong> with a number that makes more sense, i.e. <strong><i>0.98</i></strong>
(A number that is less than the current dollar price of said asset).  I have it set to 1.05 (105%) in order to force the arbitrage function to execute during testing.
</p>

![image](https://user-images.githubusercontent.com/72562693/223864467-6840e485-ff97-48ac-ad19-be3b8294edc6.png)



<h2>Running the Bot</h2>

```
cd arb-bot
```

```
cargo run
```

<p>
You should get a result similar to this (minus the insufficient balance error if you have money on binance.us to execute the buys)
</p>

![image](https://user-images.githubusercontent.com/72562693/223862910-aa6bacf6-4f33-47ed-b937-42b40a0d7aea.png)


<h1>USE AT YOUR OWN RISK</h1>

