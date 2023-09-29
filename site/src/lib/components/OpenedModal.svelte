<script lang="ts">
    import type { GameCell } from "$lib/types";
    import { getModalStore } from "@skeletonlabs/skeleton";
    import type { ArrayLog } from "secretjs";

    const modalStore = getModalStore();

    const meta = $modalStore[0].meta!;
    const cell = meta.cell as GameCell;
    const logs = meta.logs as ArrayLog ?? [];

    
    const openingLogs = logs.filter(l => l.type == "wasm-cell-opening"); 

    const canOpenNext = openingLogs.find(log => log.key == "can_open_next_at")?.value ?? "0";
    const nextOpenCell = openingLogs.find(log => log.key == "can_open_cell_at")?.value ?? "0";
    const cellScore = openingLogs.find(log => log.key == "cell_score")?.value ?? "0";
    const userScore = openingLogs.find(log => log.key == "user_score")?.value ?? "0";
    const rewarded = openingLogs.find(log => log.key == "rewarded")?.value == "true";

</script>

{ #if  cell && logs.length }
    <div class="bg-purple-100 p-3 px-5 rounded border border-black flex flex-col">
        <h6 class="font-bold flex justify-center">Successfully opened new cell</h6>
        <hr class="dropdown-divider my-3">
        <!-- grid with two colums -->
        <div class="grid grid-cols-2 gap-1">

            <span class="font-bold">Cell ID</span>
            <span>{cell.id}</span>

            <span class="font-bold">Opened at</span>
            <span>{cell.open_at.toLocaleString()}</span>

            <span class="font-bold">User open next at</span>
            <span>{new Date(Number(canOpenNext) * 1000).toLocaleString()}</span>


            <span class="font-bold">Cell open next at</span>
            <span>{new Date(Number(nextOpenCell) * 1000).toLocaleString()}</span>

            <span class="font-bold">Cell score</span>
            <span>{cellScore} / 255</span>

            <span class="font-bold">User score</span>
            <span>{userScore} / 255</span>

            <span class="font-bold">Rewarded</span>
            <span>{rewarded ? "Yes" : "No"}</span>
        </div>

        <hr class="dropdown-divider my-3">

        <div class="flex items-center justify-center">
            <button class="btn font-bold" on:click={() => modalStore.close()}>Ok</button>
        </div>
    </div>
{/if}
