# Meme Museum

## API

```typescript
// method: create_meme
type CreateMemeInput {
  title: string,
	url: string,
	museum_name: Sstring,
}

// method: get_meme
type GetMemeInput {
	id: number // u64
}

// method: get_meme
type GetMemeListInput {
	// No inputs
}

// method: donate_meme (payable)
type DonateMeme {
	id: number // u64
}
```

## Deploy contract

Compile to wasm:

```
cargo build --target wasm32-unknown-unknown --release
```

Dev deploy:

```
near dev-deploy ./target/wasm32-unknown-unknown/release/meme_museum.wasm
```

In this case, the created account/contract is `dev-1660968562799-63169581314581`.

Add the contract to ENV to use easily:

```
# SC = Smart-Contract
SC=dev-1660968562799-63169581314581
```

## Call a method

```
near call $SC create_meme '{"title":"Never gonna give you up","museum_name":"Bellas Artes","url":"https://imagenes.20minutos.es/uploads/imagenes/2020/06/19/rick-astley-en-su-cancion-never-gonna-give-you-up.gif"}' --accountId kranfix.testnet
```

This call will log the following:

```
Scheduling a call: dev-1660968562799-63169581314581.create_meme({"title":"Meme de Frank","museum_name":"Mafia","url":"https://imagenes.20minutos.es/uploads/imagenes/2020/06/19/rick-astley-en-su-cancion-never-gonna-give-you-up.gif"})
Doing account.functionCall()
Receipt: 5TGdTQsqDzxGrUEbrFNFyypw5BjJtCVY61PTMcX3FwjE
        Log [dev-1660968562799-63169581314581]: Meme 0 created by  in Mafia museum
Transaction Id DCYHzbawxWvAdCMYwgyWwTZAZFq1atAzm5WtREUhP9x8
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/DCYHzbawxWvAdCMYwgyWwTZAZFq1atAzm5WtREUhP9x8
''
```

Let's call a view method:

```
near view $SC get_meme_list ''
near view $SC get_meme '{"id":0}'
```

## Watch subaccount in the wallet

See this [answer in stackoverflow](https://stackoverflow.com/questions/71238889/how-to-import-an-account-into-the-near-wallet-using-only-the-private-key-no-see)
