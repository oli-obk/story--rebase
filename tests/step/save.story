//@ compile-flags: --dump-save
//@ check-pass
entrance
## entrance
You enter a dark cave
out: gtfo
corridor: go deeper

## corridor
It's dark and your steps echo far ahead of your
deeper: walk on
entrance: return

## deeper
You enter a large cavern with glowing moss on the walls.
corridor: return
deeper2: walk on
crawlspace: explore a small crawlspace to your right
