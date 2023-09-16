//@ revisions: go_to_deeper go_to_void invalid_selection parser_error
//@[go_to_deeper,go_to_void] check-pass
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
