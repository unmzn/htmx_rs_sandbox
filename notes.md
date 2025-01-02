# 2024-12-16 Mon
## INIT

### Stack research:
Interesting hosting option: https://www.shuttle.dev/blog/2023/10/25/htmx-with-rust
HTMX
Alpine.js or Hyperscript or surreal
tailwind css
components (not $300): HyperUI, FloatUI, TailGrids
icons?: 
- https://www.svgrepo.com/
- lucide.dev
- https://remixicon.com/
- https://tabler.io/icons
- https://phosphoricons.com/
- https://feathericons.com/
- https://fonts.google.com/icons
- https://css.gg/
practice responsive design
progressive web app?
progressive envancement / js free?
can I doo SSR with HTMX?
- re-render full (new) page on click as fallback if JS disabled?

Axum
Maud
with pulldown_cmark and ammonia?

#### read later:
https://hypermedia.systems/introduction/



## server stuff:
nginx research
mTLS?
rmfakecloud


### nginx


# 2024-12-19 Thu
## starting thoughts
- [ ] nginx/server config
- [ ] rust-embed
- [ ] css file
- [ ] db integrations

# 2024-12-21 Sat
## EOD thoughts
new page
something dogfoodable w/ myself and Rae
- spreadsheet replacement?
    - finances?
    - inventory?
    - gardening?
    - chores?
    - gymtrac
- rules
- planning somehow?
simple password and/or magic link sign in
responsive / mobile first(ish)
DARK MODE

### Spreadsheet replacement reqs:
dashboarding
flexibility
- subdivisibility / mergability
alerting
dynamic formulas?


#### time for time
login
views:
- add entry
- home/dashboard

##### possible overengineering:
- <2 users in a cluster?
back again
needed feature I think


# 2024-12-23 Mon
## using maud
higher-order templates
markup params


## stack building
### security
#### password hash: argon2
https://docs.rs/argon2/latest/argon2/

##### fancy but hard
hmac or shnorr something
https://neilmadden.blog/2024/09/18/digital-signatures-and-how-to-avoid-them/

#### just use JWT? webauthn?
https://github.com/kanidm/webauthn-rs
- I like this one
- skip passwords?
- Can I support multiple passkeys per account?
    - Docs seem to allude to this
- https://github.com/kanidm/webauthn-rs/tree/master/tutorial

# 2024-12-23 Mon
## EOD:
next step is running tutorial
want to figure out WASM integration
can I make it play nicely with htmx?
interesting yew example

still want to work on (meta?) html templating
- macros?
- function based / class list composition
    - impl trait input type?
        - ctx? model?
        - to_header / to_form / to_page etc.?
            - procedural macros?
                - customize with config file?
    - class generating function from parent?
        - from child??
        - can Hyperscript do this?
            - wrong layer?

# 2024-12-25 Wed
## eod
"iframes" or real iframes
responsive design subpages flexible for different contexts
procedural macro codegeneration?
"real" iframes to embed content from the wider internet

### living text
seems to go really well with htmx etc.

### DAW stuff
couldn't get them to make any sound
installed 2
1 non-free
2 rust-synth exists, didn't try yet

# 2024-12-29 Sun
found maud doc thing:
```rust
const GITHUB: &'static str = "https://github.com";
html! {
    a href={ (GITHUB) "/lfairy/maud" } {
        "Fork me on GitHub"
    }
}
```

## try
use 2^n grids to represent tree
nav with htmx / be / hyperscript
even responsive?

smallest (1,2)
    x
x   |   x

next (1,2,4)
                    x
                x   |   x
              x | x   x | x


next (1,2,4,8)
                    x
        x           |          x
    x   |   x              x   |   x
  x | x   x | x          x | x   x | x
last_2 (1,2,4,8,16)

1 3 9
1 4 16
the ancients would approve
1 2 6 24
dynamic depending?
appealing

## idea2
dynamic features => dynamic schema-per-tenant
- ui to configure
- admin level audit log
- "base schema"
    - tenant level audit log
    - user

## idea3
tree-shaking css/js compiler built into maud or integrating with it somehow
- simplest: pure compiler tightly integrated
    - syntax draft 1: `pure-class(grid) (|&-)(set ops) (input_variable)`
        - rust HashSet supports all desired operations
    - then tight integration with maud too? output Markup maybe? macro_rules enough? proc-macro?

## cool find
use dashes in css selectors
- `|= attribute selector`
    - http://www.w3.org/TR/CSS21/selector.html#attribute-selectors

## other
look into ol' bootstrap?
htmx exaples be dope: https://htmx.org/examples/
- may be useful (backend): https://github.com/paultuckey/example-todo-app-rust-htmx
hscript: https://hyperscript.org/cookbook/


# 2024-12-30 Mon
## DAW: promising results with LMMS
## HTMX:
has a JS api, could be useful in advanced scenarios

### giant svg?
yes
canvas is super js-y
svg is markup-y
said to be the best first choice


# 2025-01-01 Wed
## svg icons (said to be OSS):
https://flowbite.com/icons/
## svg optimizer
https://github.com/svg/svgo
## svg animate w/ css:
https://css-tricks.com/animating-svg-css/
