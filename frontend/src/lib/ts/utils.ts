export const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export const waitUntil = async (
    isOkayCheck: () => boolean, 
    interval: number = 100, 
    giveUpAfter: number = 4000
) =>  {
    let timer = 0;
    while(!isOkayCheck()) {
        if(timer > giveUpAfter) {
            throw new Error("waitUntil timed out");
        }
        await sleep(interval);
        timer += interval;
    }
}