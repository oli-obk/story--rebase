//@ check-pass
loot room

## loot room
There is a chest in front of you
open: Open it
knock: Knock on it
smash: Take a swing at it with your trusty sledgehammer
leave: Leave

## knock
The chest opens and shows a serrated set of teeth
leave: Run away
smash: Take a swing at it with your trusty sledgehammer
open: Try to talk it into giving up its riches

## open
You are looking at an open chest filled with riches
[inventory.gold += 10]: Grab some gold
{1}[inventory.shiny_sword += 1]: Grab the shiny sword
leave: Leave

## leave
You hear scurrying behind you
knock: Turn around
run faster: Run faster

## run faster
The last thing you see is a set of teeth closing in front of you
