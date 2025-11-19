# Strongest Cubical

## 🧱 The Cubical Dungeon Guardian Speaks

A dim lantern flickers above rows of perfectly aligned stone cubes.
From behind one of the cubicles, a hunched figure emerges—half-scribe, half-daemon, all SQL.

“Ahh… a wanderer! You’ve entered the Cubical Dungeon, where data goes to die—or be optimized.
I am the Warden of Cubes, caretaker of rows and columns long forgotten.
You seek passage? Then face my challenge.”

He slams a stone tablet onto a slab between you.

“Monsters lurk in each cube. Some weak, some mighty, some too embarrassed to list their hit points.
Your challenge: for every cube, determine the single strongest monster that dwells within it.
When you believe you’ve solved this riddle using SQLite… run the Test Ritual to see whether your answer is worthy.”

The Warden fades back into the cubicles, whispering:
“Migrate wisely, wanderer…”

## 🧩 The Challenge

After running the migrations below, write a query that produces a table with:

    cube_id | cube_name | monster_id | monster_name | hp

…containing the strongest (highest HP) monster in each cube.
If a cube has no monsters, it should not appear (inner semantics).

Create your solution as a view named strongest_monsters.


## Technical Instructions 

    sqlite3 test.db < migration.sql
    sqlite3 test.db

