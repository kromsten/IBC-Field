import { readFileSync, writeFileSync } from "fs";
import { Config } from "./types";


export const loadConfig = () : Config => {
    return JSON.parse(readFileSync('tests/config.json', 'utf8'));
}

export const saveConfig = (config : Config) : void => {
    const json = JSON.stringify(config, null, 4);
    writeFileSync('tests/config.json', json, 'utf8');
}
