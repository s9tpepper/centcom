# Features

## Name Ideas
- Protocall
- Protocol
- Mercury
- Herald
- A P.I.
- IPA
- Hazy
- Triple Hazy
- Probe
- Mo
- Larry
- Curly
- Nyuk Nyuk
- Omen
- Prophet
- Seer
- Oracle
- Druid




## TODO

### Options
[ ] (MVP?) Set up GitHub integration in the options?
[N] Allow keybinding customizations?? (not really sure about this one)
[x] Themes for UI colors

### UI
[x] Pass on cleaning up keybindings
[x] (MVP) Reserve Q for quitting app
[x] Pass on UI colors for readability
[x] (MVP) Fix bindings so they are contextual to the screen that you are on
[x] Clear project/endpoint name dialog inputs if the name is still default
[x] (MVP) Options screen to choose themes, background for responses/request bodies, etc
[x] (MVP) Flesh out the options for the app
[ ] Open In for URL/response
[x] (MVP) Fix Ctrl C in request body

### Response
Body
[x] (MVP) Save response body
[x] (MVP) Filter/Search response body
[x] (MVP) Highlight search results so you can see them as you navigate the search result list
[x] (MVP) Syntax Highlighting
[x] (MVP) Syntax Highlighting themes (tmTheme files)
[x] (MVP) Save new syntax themes to options
[x] (MVP) Update code sample and make window resize dynamically based on available space
[x] (MVP) Fix the entire response area background to be the intended bg color of the chosen theme
* [x] (MVP) Virtualized Response body view
[ ] Prettier/formatter type of integration to make responses read easier

### Projects
[x] (MVP) Switch projects
[x] (MVP) Switch endpoints
[ ] Export to Postman format
[x] (MVP) Add Project
[x] (MVP) Add Endpoint
[x] (MVP) Rename Project dialogue

### Requests
* [ ] (MVP?) Request dependencies/piping results from dependency
* [ ] (MVP?) Project/Request variables, like for tokens so you don't have to copy/paste tokens for every request
* [ ] (MVP) Code generation (curl, TypeScript/JavaScript, Rust, PHP?, Go?, Python?) [ ] Code generation plugin framework, based on OpenAPI code generation plugins/tools, maybe?
[ ] Import/Export OpenAPI?

### Text Input
[x] (MVP) Possible bug with backspace in text input not removing character

### Optimizations
[x] Update AppTheme/AppThemePersisted to use serde w/out derive to try to get rid of the need for two structs to save JSON
    Note: This didn't turn out so well, too complicated, kept duo structs
