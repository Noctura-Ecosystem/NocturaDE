# UPDATE LOGS

## SAMPLE 1
**Date:** 2026-02-20 00:12:10

- This is just a simple structure of how the compositor will be made
- The compositor will NOT be completly made in python
- It will probably be made in Rust (but c is also an option)
- The compositor has the following functions:-
    - Handles input/output devices
    - Necessary variables (like display, scence...) are stored using attributed (i will use structs in the actual version)

---


## SAMPLE 2
**Date:** 2026-02-21 20:39:48

- Finished with the template (its in pseudocode + python)
- NocturaDE will officially start being made
- Framework list decided
    - React (for shell)
    - Smithay (for Compositor)

---

## SAMPLE 3
**Date:** 2026-02-25 13:34:38

- Made a simple version of the DE in Rust
- The following features have been implemented
    - A window will open if you run the program
    - Initializes the wayland-socket
    - You can run apps (but it wont be visable as renderer isnt made yet)

---

## PATCH of SAMPLE 3
**Date:** 2026-02-25 13:53:35

- got rid of useless file: handlers.rs (this is only for future ideas)

---

## SAMPLE 4
**Date:** 2026-02-27 12:21:03

- Completed init_wayland function by adding the handle_input_event and surface_under function (both inspired by smallvil)
- features of handle_input_event include:-
    - can handle keyboard events
    - can handle pointer events (scroll wheel, pointer button, cursor)
    - can handle touch/absolute cursor position value
- surface_under was made so we can know exactly under which UI element the cursor is under

---
