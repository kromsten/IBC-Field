import { coinFromString, type SecretNetworkClient } from "secretjs";


export const sendIBCToken = async (
    client: SecretNetworkClient,
    receiver: string,
    token: string,
    amount: string,
    source_channel: string,
    memo: string = ""
) => {

    console.log("sending token", token, "to", receiver, "with amount", amount);
    console.log("source channel", source_channel);

    const res = await client.tx.ibc.transfer({
        sender: client.address,
        receiver,
        token: coinFromString(amount + token),
        source_port: "transfer",
        source_channel,
        memo,
        timeout_timestamp: String(Math.floor(Date.now()/1000) + 2 * 60)
    })

    console.log(res);

    return res;
}