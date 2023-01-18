// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

import fs from "fs";
import Path from "path";
import Shell from "child_process";
import { Modules } from "./module.base"

/*============================================================================*/

export async function ExecuteC(file: string, flags: string): Modules.ReturnType {
	// Compile, execute and remove binary
	const binary = Path.join(Path.dirname(file), Path.parse(file).name);

	const execution = new Promise<string>((resolve, reject) => {
		Shell.execSync(`gcc ${flags} ${file} -o ${binary}`, { timeout: 10000 });
		Shell.execFile(binary, { timeout: 10000 }, (err, stdout, stderr) => {
			fs.rmSync(binary, { force: true, recursive: true });
			
			if (err) return reject(err);
			if (stderr.length > 0) return reject(stderr);

			return resolve(stdout);
		});
	});

	try { 
		const data = await execution;
		return [data, null]
	}
	catch (error) {
		return [null, error];
	}
}
