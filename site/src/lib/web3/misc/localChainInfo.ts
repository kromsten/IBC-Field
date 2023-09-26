import { PUBLIC_CONSUMER_CHAIN_ENDPOINT, PUBLIC_SECRET_CHAIN_ENDPOINT } from "$env/static/public"

export const getLocalSecretChainInfo = (
    chainId: string,
    endpoint?: string,
    nameSuffix?: string
) => {

    if (!endpoint) {
      if (chainId == "test-1") {
        endpoint = PUBLIC_SECRET_CHAIN_ENDPOINT;
        nameSuffix ??= "Test 1"
      } else if (chainId == "test-2") {
        endpoint = PUBLIC_CONSUMER_CHAIN_ENDPOINT;
        nameSuffix ??= "Test 2"
      }
    }


    return {
        "rpc": endpoint!,
        "rest": endpoint!,
        "chainId": chainId,
        "chainName": "Local Secret " + nameSuffix,
        "chainSymbolImageUrl": "https://raw.githubusercontent.com/chainapsis/keplr-chain-registry/main/images/secret/chain.png",
        "stakeCurrency": {
          "coinDenom": "SCRT",
          "coinMinimalDenom": "uscrt",
          "coinDecimals": 6,
          "coinGeckoId": "secret",
          "coinImageUrl": "https://raw.githubusercontent.com/chainapsis/keplr-chain-registry/main/images/secret/uscrt.png"
        },
        "walletUrl": "https://wallet.keplr.app/chains/secret-network",
        "walletUrlForStaking": "https://wallet.keplr.app/chains/secret-network",
        "bip44": {
          "coinType": 529
        },
        "alternativeBIP44s": [
          {
            "coinType": 118
          }
        ],
        "bech32Config": {
          "bech32PrefixAccAddr": "secret",
          "bech32PrefixAccPub": "secretpub",
          "bech32PrefixValAddr": "secretvaloper",
          "bech32PrefixValPub": "secretvaloperpub",
          "bech32PrefixConsAddr": "secretvalcons",
          "bech32PrefixConsPub": "secretvalconspub"
        },
        "currencies": [
          {
            "coinDenom": "SCRT",
            "coinMinimalDenom": "uscrt",
            "coinDecimals": 6,
            "coinGeckoId": "secret",
            "coinImageUrl": "https://raw.githubusercontent.com/chainapsis/keplr-chain-registry/main/images/secret/uscrt.png"
          }
        ],
        "feeCurrencies": [
          {
            "coinDenom": "SCRT",
            "coinMinimalDenom": "uscrt",
            "coinDecimals": 6,
            "coinGeckoId": "secret",
            "coinImageUrl": "https://raw.githubusercontent.com/chainapsis/keplr-chain-registry/main/images/secret/uscrt.png",
            "gasPriceStep": {
              "low": 0.1,
              "average": 0.25,
              "high": 0.5
            }
          }
        ],
        "features": ["secretwasm", "authz-msg-revoke-fixed"]
      }
}