// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

import cors from "cors";
import express from "express";
// import config from "./config.json";
import { Execution } from "./executor";
import { Request, Response, NextFunction } from "express";

// Globals
/*============================================================================*/

export const webserv = express();
export const port = 4242;

// Middleware
/*============================================================================*/

webserv.use(cors());
webserv.use(express.json());
webserv.use(express.urlencoded({ extended: true }));
webserv.use((err: any, req: Request, res: Response, next: NextFunction) => {
	if (err.statusCode === 400 && "body" in err)
		res.status(400).send({ status: 400, message: err.message });
});

// Routes
/*============================================================================*/

webserv.post('/playground/', (req, res) => {
	const code = req.body.code as string;
	const flags = req.body.flags as string;
	const language = req.body.language as string;

	// Check request
	if(!req.is("application/json"))
		return res.status(400).json({ result: null, error: "Incorrect content type!" });
	if (code == null || language == null || flags == null)
		return res.status(400).json({ result: null, error: "Malformed body" });

	// TODO: Get from config.
	// TODO: Check from which domain the request came from.
	// if (req.headers.origin && !req.headers.origin.includes(".codam.nl"))
		// return res.status(403).json({ result: null, error: "Non-valid origin" });

	// TODO: Probs add a few more checks here for unwanted requests.

	// Find module
	let module = Execution.modules[language];
	if (module == undefined)
		return res.status(404).json({ result: null, error: "Unsupported Language!" });

	console.log(`[Playground] Request with: ${language} | Flags: ${flags.length > 0 ? flags : "None"}`);

	return Execution.run(module.executor, code, flags, module.extension)
	.then((output) => res.status(201).json({ result: output, error: null }))
	.catch((error) => res.status(422).json({ result: null, error: error.message }));
});


// Entry point
/*============================================================================*/

webserv.listen(port, () => console.log(`[Playground] Running on: ${port}`));
