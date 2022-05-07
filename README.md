# luau-explorer

Work in progress. Slash commands are horrible to work with, so much so I'm not feeling motivated to finish this.

Similar to Godbolt, but for Luau on Discord integrating with Luau's compiler API.

We'd like to:

1. take an arbitrary fragment and analyze it, returning a list of type errors and lint errors + parse errors if any
2. take an arbitrary fragment and run it (this is safe due to sandboxing but we do need to place an upper bound in runtime)
   * possible further restriction: global limit in concurrent VMs? and a limit in concurrent VMs per guild?
3. take an arbitrary fragment and tell the user the inferred type of some particular symbols
4. take an arbitrary fragment and return the bytecode of it
5. take an arbitrary fragment and visualize AST as JSON
6. take an arbitrary fragment and show autocomplete suggestions (up to N limit)
