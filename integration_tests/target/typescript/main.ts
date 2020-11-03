import { MAIN_CLASS } from "./CODEGEN_DIR";
import * as readline from "readline";

readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false,
}).on("line", (line) => {
    const value = JSON.parse(line) as MAIN_CLASS;
    console.log(JSON.stringify(value));
})
