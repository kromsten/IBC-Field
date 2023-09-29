import { PUBLIC_CONTRACT_ADDRESS } from "$env/static/public";
import type { SecretNetworkClient,  } from "secretjs";
import { OrderBy } from "secretjs/dist/grpc_gateway/cosmos/tx/v1beta1/service.pb";

const eventBase = "execute.contract_address='" + PUBLIC_CONTRACT_ADDRESS + "' AND "

export const getOpenCellLastInteraction = async (
    client: SecretNetworkClient,
    address: string,
    cell_id?: string
) => {

    let eventQuery = eventBase + "fungible_token_packet.sender='" + address + "' "
     + "AND wasm-cell-opening.contract_address='" + PUBLIC_CONTRACT_ADDRESS + "' "

    if (cell_id) {
        eventQuery += "AND wasm-cell-opening.cell_id='" + cell_id + "' "
    };

    const txs = await client.query.txsQuery(
        eventQuery,
        undefined,
        undefined,
        OrderBy.ORDER_BY_DESC
    )

    console.log("TX QUERY:", txs)
    /* client.query.txsQuery("")
    const res = await client.q(client.key, {
        get_last_interaction: {}
    });
    return res.last_interaction; */

    if (txs.length == 0) {
        return null;
    }

    return txs[0];
}