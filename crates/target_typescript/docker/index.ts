// This import exists only to ensure that TypeScript does type-check the
// generated code. Ultimately, for TypeScript there is no "runtime" aspect to
// code generation. The Dockerfile that runs this file just runs `cat`, because
// code like this:
//
// console.log(JSON.stringify(JSON.parse(stdin) as MAIN))
//
// Is rather pointless.
import { MAIN } from "./jtd_codegen_e2e";
