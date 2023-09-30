# IBC-Field

Cross-chain game-like project that heavily utilises Secret Network as source of verifiable random numbers, private storage and computations but the abstract all the usage away with the help of IBC-hooks

Users can interact with cells to open them and have a chance of winning a bigger prize. The amount of interaction for each cell and each users is time-limited, however users can purchase powerups that allow them to pass the restrictions

## Installation

You'd need amd64 based processor for running a local-secret, `docker-compose` (or latest docker with compose command)  for setting a local environment.
In addition to that you'd need a javascript runtime and package manager. The examples use `node`, but `yarn`, `pnpm`. The latests version of `bun` have not yet been tested, but there has a bit a lot of issues with cryptography of `secretjs` historically

### Local IBC-setup with two chains a relayer
```
cd ibc-setup
docker-compose up --build 
```

You might need to wait a minute until hermes finishes creation of an IBC channel between two local chains before proceeding to the next steps

### Smart contract
Build a contract ether using `cargo build` or using docker contract optimizer.
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
Re-running the test again might help in some cases where there were delays during IBC-steps

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

You can run a develpemnt vite server using
```
npm run dev
```
which will have the app running on
http://localhost:5173




