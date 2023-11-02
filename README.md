# IBC-Field

Cross-chain game-like project that heavily utilises Secret Network as source of verifiable random numbers, private storage and computations but the abstract all the usage away with the help of IBC-hooks

Users can interact with cells to open them and have a chance of winning a bigger prize. The amount of interaction for each cell and each users is time-limited, however users can purchase powerups that allow them to pass the restrictions

Public Demo deployed using Akash is coming soon after establishing IBC channels on testnets

## Interacting from Akash

Thanks to the latest upgrade we can use IBC-Hooks to ineract with contracts on Secret Network from Akask. An akashian doesn't need to hold any $SCRT tokens to pay for the gas and. Users only need to submit an ibc-transfer transactions with a certain payload, gas for each is paid with native $AKT tokens or any other token accepted by validators of the Network 

### Transaction Structure

All of the transactions coming from Akash must follow the "IBC-Hook" standard and formatted accordingly. See the exact schema and notes regarding Secret Network implementation [here](https://github.com/scrtlabs/SecretNetwork/blob/master/x/ibc-hooks/README.md)

Different actions a user can take to interact with the app are all encoded inside `memo["wasm"]["msg"]` field, so there is no need for other type of transactinos. 

When it comes to querying the state of an app we can query the contracts directly using an RPC node of Secret Network. Querying doesn't require any gas usage so this way we can do it directly without a prompt and additional actions from a user for seemless expierience

### Authentication

The app uses amino signatures for authenticating Akash users interacting with the contract over IBC. The design is heavily inspiried by [permits](https://docs.scrt.network/secret-network-documentation/development/secret-contract-fundamentals/access-control/permits) designed for Secret Network. 

In this scenario signatures are sent over IBC and being exposed to the world in plain text. That open some possibility for front-running and possible impersonatation of users inititating IBC transactions

For that reason the signatures should be limited and only used in temproral manner. For example we can include a timestamp or a block height in the data structure to be signed and validate that the signatures are fresh. Another scenario possible for Secret Network is to have the authenticating signature encrypted to send it over IBC and to decrypt it inside a contract

In this case we don't deal with sensitive data and take a really simple approach. We only allow a signature to be used once esssentially making them work as one-time passwords. After a succesful transaction the signature will be invalidated and can no longer be used for acting on behalf of a user

### Contract Actions

The contract structure itself doesn't require significant changes to allow users of Akash Network to interact with it. The only change we need to introduce is to require users to include a signed permit defined in [secret-toolkit](https://github.com/scrtlabs/secret-toolkit/tree/master/packages/permit) included in each action message:

```rust
ExecuteMsg {
  Action {
    ...
    permit: Permit
    ...
  }
}
```

This is due to the fact that `sender` field  from `MessageInfo` we are typically using is not reliably in general and in case of Secret Network removed completely

See the exact list of messages a user can take [here](/contracts/random/src/msg.rs#L16)

### Examples

See the examples of how to interact with the contract over IBC from code of [a demo site](/site/src/lib/web3/contract.ts#L21) and [testing suite](/ibc-setup/tests/tests/field.test.ts#L46) in this repository

Keep in mind that here we have `SecretNetworkClient` only as a typescript type annotation. It is a helping wrapper from `secretjs` package for interacting with blockchain nodes and it is completely compatible with Akash Network (within our use case). Both `akashjs` and `cosmjs` can be used as well. `secretjs` was picked over them due to its additional helping functins related to IBC and also due to the fact that we would need it anyway for querying operations, so this way we have less dependencies.

Authenticating permits on client side is also relying on helping functions of `secretjs`. See [here](site/src/lib/web3/index.ts#L41). It is howeverusing standardised amino signature underneath so it can be replicate with other libraries and any wallet

## Installation

You'd need amd64 based processor for running a local-secret, `docker-compose` (or latest docker with compose command)  for setting a local environment.
In addition to that you'd need a javascript runtime and package manager. The examples use `node`, but `yarn`, `pnpm` should work as well

### Local IBC-setup with two chains and a relayer
```
cd ibc-setup
docker-compose up --build 
```

You might need to wait a few minutes until hermes finishes creation of an IBC channel between two local chains before proceeding to the next steps

### Smart contract
Build a contract ether using `cargo build` or using docker contract optimizer
Move the wasm file into
`ibc-setup/tests/tests/contract_code`

### Integration tests (helpful for contract instantiation)

Proceed into `ibc-setup/tests` folder and install the dependencies using
```
npm install 
```

Add a `.env`with the following fields and values an example

```
SECRET_MNEMONIC="grant rice replace explain federal release fix clever romance raise often wild taxi quarter soccer fiber love must tape steak together observe swap guitar"
SECRET_CHAIN_ENDPOINT="http://localhost:1317"
SECRET_CHAIN_ID="test-1"
SECRET_TOKEN="uscrt"

CONSUMER_MNEMONIC="jelly shadow frog dirt dragon use armed praise universe win jungle close inmate rain oil canvas beauty pioneer chef soccer icon dizzy thunder meadow"

CONSUMER_CHAIN_ENDPOINT="http://localhost:2317"
CONSUMER_CHAIN_REST="http://localhost:2317"
CONSUMER_CHAIN_RPC="http://localhost:36656"


CONSUMER_CHAIN_ID="test-2"
CONSUMER_TOKEN="uscrt"
```


After that you can run the following command
```
npm test run setup
```
It will check the validity of your configuration and deplloy the contract to the first local chain
In some cases there are delays during IBC-steps, which causes the automatic setup to crush. Re-running the tests again until all the tests are passed should resolve the issue 

The command will save the data on the uploaded contract into `tests/config.json` file. You can use it to fill envieromental variables in the following steps

To have the rest of the intregration tests executed you can use
```
npm run test index
npm run test field
npm run test powerups
```

### Frontend

Proceed into `site`folder and install the project using

```
npm install 
```

Fill the .env file following the example
```
PUBLIC_SECRET_CHAIN_ENDPOINT="http://localhost:1317"
PUBLIC_SECRET_CHAIN_ID="test-1"
PUBLIC_SECRET_TOKEN="uscrt"


PUBLIC_CONSUMER_CHAIN_ENDPOINT="http://localhost:2317"
PUBLIC_CONSUMER_CHAIN_ID="test-2"
PUBLIC_CONSUMER_TOKEN="uscrt"

PUBLIC_SECRET_CHANNEL="channel-0"
PUBLIC_CONSUMER_CHANNEL="channel-0"

PUBLIC_CONTRACT_ADDRESS="secret1gyruqan6yxf0q423t8z5zce3x7np35uw8s8wqc"
PUBLIC_CONTRACT_CODE_HASH="4870a18d4fe8fbf36abc842ab1e54400e24c2c6618486cdaf801ab8d5f725f6f"
```


Now you can run a development server powered by `vite`using
```
npm run dev
```

The app should be running on
http://localhost:5173




