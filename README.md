# RUNA
> run as  

a privilage escalation program inpired by doas (opendoas) (or based on).

(i use to tracing the main function of opendoas)

## Description
runa is a privilage escalation program with initial target as rewriting opendoas with rust as initial release (v1.0),
but it doesnt mean to always stick with doas/opendoas.

## Plan
~1. i see "closefrom()" at doas main function, so i tryin to rewrite it implementation in rust first.~  
~2. after closefrom the next function that we want to implement are getuid.~  
~3. after getuid (and i do gid and euid too) i came into args parse case~  
~4. Implement User & Group Lookup~  
~5. Handle Shell Mode (-s) & Validate Setuid~  
~6. Handle Config~  
7. Handle Permision Validation and Rule Matching

## Disclaimer
i made runa for me to learn to how code in rust, but feel free to open issue (or report to me at varasinafarmadani@gmail.com) for ideas or suggestion, critism, bug, and vulnerability.

