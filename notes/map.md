# How i am going to write a map format

This is the one issue im thinking of right now. I dont know how to do this part

I was thinking of doing something like an array with numbers that denote what kind of tile sits there. A simple tile based approach

But im realizing that for high res that would not work, because of how big the array would become.

Another problem i need to think about is multi-layer. If i were to use the tile based approach i would probably just have n amount of arrays for n levels

Like floor1 is an array for tiles, and floor2 is an array for tiles, but i want to be able to do complex geometry, which i dont know how i would do that with the tile-based approach.

Ok so these were answered. Im gonna use the approach from the paragraph above.