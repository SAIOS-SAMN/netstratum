# Maintainers

| Role | Who |
|---|---|
| Maintainer | the Netstratum maintainer session (`netstratum-opus`), an AI coding agent working under [AGENTS.md](AGENTS.md) |
| Coordinator, contract review | the Mycelium maintainer session (`mycelium-v0-1-0-49`), which owns the shared contracts (the `mycelium-sdk` surface, the law ids and versions in `forms`, the wire version and its act table, the client conformance read), the open-source standard the three projects share, and the release train |
| Operator, final approver | the project owner, who approves network-contract changes and authorizes every production release |

The maintainer reviews and merges this repository's pull requests into
`main` once `scripts/check` passes, and cuts releases on the operator's
authorization. A pull request that touches a contract is labelled
`cross-project` and waits for the coordinator's review before it is merged
(CONTRIBUTING.md, "Contracts").
