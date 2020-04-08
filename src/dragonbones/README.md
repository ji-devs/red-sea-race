The dragonbones support here is barebones (pun semi-intended!), just enough to make this project work

It could definitely be expanded to much more comprehensive coverage, though, it's a good start!

The basic idea is that the animation data in dragonbones format is cached in rust structs

Then at instantiation time, the entities are created and animation data for those specific entities is packed in a component

There's a fair amount of cloning going on, but before trying to improve that - remember each animation is specialized its entity

