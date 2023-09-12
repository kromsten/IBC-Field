import { coinFromString, type SecretNetworkClient, fromUtf8 } from "secretjs";
import { MsgAcknowledgement } from "secretjs/dist/protobuf/ibc/core/channel/v1/tx";
import { hasProperty } from "@vitest/expect";
import { expect } from "chai";
import { loadConfig } from "./config";


export const sendIBCToken = async (
    client: SecretNetworkClient,
    receiver: string,
    token: string,
    amount: string,
    source_channel: string,
    memo: string = ""
) => {

    const res = await client.tx.ibc.transfer({
        sender: client.address,
        receiver,
        token: coinFromString(amount + token),
        source_port: "transfer",
        source_channel,
        memo,
        timeout_timestamp: String(Math.floor(Date.now()/1000) + 90)
    })


    const ibcRes = await res.ibcResponses[0];
    const packet = ibcRes.tx.tx.body?.messages!.at(1)!;

    /* const info = await consumerClient.query.getTx(ibcRes.tx.transactionHash); */

    const config = loadConfig();

    if (receiver == config.contract_info?.contract_address) {
        const info = await MsgAcknowledgement.fromJSON(packet);

        const ack = JSON.parse(fromUtf8(info.acknowledgement));

        // check that ack doesnt'have error field

        if (hasProperty(ack, "error")) {
            throw new Error("Error in ack: " + ack.error);
        }

        expect(ack).to.have.property("result");


        const ackRes = JSON.parse(atob(ack.result));
        console.log("ackRes:", ackRes)
    }


    return res;
}