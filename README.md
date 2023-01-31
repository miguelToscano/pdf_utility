The PDF Utility canister will allow other canisters on the IC to generate PDF documents.

Requirements:

- Use GitHub - dabega/genpdf-rs 3 to expose PDF generation functionality.
- Allow a canister to push images/fonts/text blocks into the canister(Must support >2MB files, perhaps implement pipelinify schema - GitHub - skilesare/pipelinify.mo: Move data chunks between canisters 1)
- Create a candid object notation for describing a PDF using the features supported by genpdf-rs.
- Ability to references named, pushed files/fonts.
- Provide performance tests and explanations of the limits of the canister’s ability to generate a PDF given cycle limits.
- If the library requires multiple rounds to generate a reasonable PDF, consider implementing pipelinify with timers to automate the task across multiple rounds.
- The canister should be created such that it can be deployed and assigned an allow list of callers.

It should also work as a library for other rust canisters that want to include the functionality as a part of another canister without compile errors.

With this version of a the canister, the user that wants PDF generation should deploy their own Utility canister so that they can manage their own cycles.

Upon completion of this bounty the developer will be auto qualified to implement any future bounty that may include an attempt to wrap the service as a broader utility complete with its own tokenomincs and potentially and eventual deployment via SNS or other DAO structure.