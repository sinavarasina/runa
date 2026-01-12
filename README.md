# RUNA
> run as  

a privilage escalation program inpired by doas (opendoas) (or based on).

(i use to tracing the main function of opendoas)

## Plan
~1. i see "closefrom()" at doas main function, so i tryin to rewrite it implementation in rust first.~  
2. after closefrom the next function that we want to implement are getuid.
