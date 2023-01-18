// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

import fs from "fs";
import tmp from "tmp";
// import crypto from "crypto"
import { Modules } from "./modules/module.base";
import { ExecuteC } from "./modules/module.c";

/*============================================================================*/

export namespace Execution {
	/** Map to associate languange with the correct executionModule */
	export const modules: {[name: string]: { executor: Modules.Function, extension: string }} = {
		"c": {
			executor: ExecuteC,
			extension: ".c"
		},
	};

	/**
	 * Spawns a child process for the given module and executes the code.
	 * @param module The specified module to run
	 */
	export async function run(module: Modules.Function, code: string, flags: string, extension: string): Promise<string> {
		return new Promise<string>((resolve, reject) => {
			tmp.file({postfix: extension }, async (err, path) => {
				if (err) return reject(err.message);
	
				// Write source code into tmp file.
				fs.writeFileSync(path, code);
	
				// Execute it
				const [output, error] = await module(path, flags);
				if (error) return reject(error);
				return resolve(output!);
			});
		});
	}
}
