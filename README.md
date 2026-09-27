# Your text, corrupted

This is a dumb little CLI I made for fun.

It takes two arguments.  The first is mandatory, the text to be corrupted.  The second is optional, a number that specifies the corruption level (how many diacritics it should add to each letter)

A few examples:

Default corruption level (8):
```
❯ icorrupt "Hello World"
H͇ͫ̀ͤͨ̔̏͡ë̷͇̱̭̿̔̾l̵̫̬̺̃̈̏̇l̟̖̑͑̿ͧ͒̕o̼̙̳̫ͫ͐͌͡ W̫ͦ̈́͒͗͑ͪͮo̸̶̻̳̭ͥͪ͡r͇̣̥͗ͯ͊ͩͅḽ̷̡͚͍̖͗́d̢̮͉͖̀́͐͏
```

Smallest corruption level:
```
❯ icorrupt "Hello World" 1
H̉e͐l̵l͊o͔ W̳o͚rͦl͢d̨
```

You can pass it pretty much any length of string:
```
❯ icorrupt "I am the very model of a modern major general. I've information vegetable, animal, and mineral. I know the kings of England, and I quote the fights historical"
I̭̖̾̽̃̒͐͞ a̴̧͍͕̰͔͒̾m̛͚̦͇̠̮̊͌ t̸̖͂́ͪͭ̔ͩh̗̺̲̥̭ͬ͌ͣe̫̗̟̬͈̓͗̌ v̸̛͈̺̀͑͜͡ę͚̣͐̒͋̇͢ṟ̷͔͈ͧͭ̚͝y̛̥̦͈ͥ̀͊͟ m̢̹̥̖̊̂̉ͣo͓̤̖̦̲̟̫̓d̶̴͖̱͎̂̐̍e̷̖̓̽̒̅͑͟ḽ̶ͯ̓͂ͫͦ͠ o͚͎̙̒ͩ̄̈̾f̶̹̰͉̊ͭͧ͘ a̛͔̜̞ͩ̍͑̑ m͙̻̙͐̀ͬ͡͠öͣ̒̓͏͍͎͗d̶̨̙̗̒̀͒͜e̱̦͖͔̥ͯ͟͝ŕ̵̫́̉̄ͮ̓n̥̣̔̓ͣ͒̇͞ m̘̩ͮ͛̓̌ͦͨa̶͖ͤ͌̑ͩ͢͞ĵ͔͇͉̞̔̒̏o̡͖̦̜̼ͬ͊̄r̪̽̓͆̈́ͫ̈͡ ģ̛͐̈́̄́ͪ͡e̶̴̡̼̣̜̿͂ņ̭͍̰̬̳ͥͧe̷̝͕̺̼ͤͦ͘r̳̯͙̐̈́ͦ͞͠a̵̜̪ͩ̚͏̽ͤļ̻͇̽̉ͤ͘͠.̗̜͙̩͔̑͠ͅ I̭̖͉͂̉̓ͮͣ'̱͓̭͕̻̼͌̽v͇̠͖̭̂͂̐̋è̪̣͎͊͋͟͞ i̧̗͌̂̋ͯ̇̚n̴̵̼͉̬̂̈́ͦf̳͇̣̗̪̈́ͤ͘ǫ̭͖͒ͦ̆͜͝r͙̠̜̲ͩͦ̎͘m̵̸̙̣̱͆͘͞a͎̥̜ͪ͒́̾̓ț̻̫̲̬͕̂́i̼͇̲̺̋ͥͯ͢o̭ͩ́̓̀̾͏̗ň̢̖̫͕͊̋̚ v͙̳̰͐̒̄̉̊e̶̜̣͏̧̺ͥ̓ģ̤̲͉ͭ́ͦ̇e̠͂̓̃̽͏̯̥t̛͓̥̞̤̂ͥ͜à̜͇̙͑ͩ̓̊b͇̥̒ͫ͋̕͢͜l̴̩̺̝̂ͬ͘͝e̺̤̻ͭ͑̒̍̚,̱̖͕̠ͨͭ̍ͪ a̢̤̘͔̐̆̇̚n̶̻ͥ̽͂ͣ̇͞i͔̟̹̜̿̂ͬ͠m̡̺͈̫͌̽͋ͅả̝̻ͯ̃̐͏̖l̴͚̓ͧ̂̏ͬ͊,̳̖͕̭͋̽ͭ͟ à̳̟ͩ̈́͌̒̕n̴̺̯̈ͯ͛̅͞d̦̗ͬ͏͖̮̽̏ m̷̰̈̀͋̓̕͝ḯ̭̻̪͙͑̌̃n̵͉̣ͬͯ̃̍͝e̪̱̜͗̄͛͐̽r̵̛̭͊ͤ͂̈͜å̟̲̝ͩͥ̆͟l̞̥̜̤͙͋͌̕.̨̠̜̱͕̦̉̚ I͚̻ͨ̈̈́̄̽͘ k̷̴̛͉ͮ̌̈́̅n̢̻̦̖̫͛̕̚ȏ̺̼́͐ͨͩ̚w͇̿͊͐͏̸ͬͅ t̝̙̖ͪ͒̔ͭ͟ẖͥ͊ͪ̓̈̕͢ě̱̬̊̾͋̿ͣ k̘̪̙̲ͩ̓̚͟i̶̛̻̤̝͒̐̿n̬͔̖̽ͦ̿̓͠ģ̴̰̟͆̈͂̌s̸̨͉͇̦͔̎ͮ o̢̫̯̬̻̓͞͝f̶̠̹̔̽̓̏͟ É̯͇̭̘͈̈́ͭǹ͎ͦ̂ͣ̉̅͛g̢͍̤̃͑̅̍́l̘̣̯͆͛̍̉̎ḁ̜̠̼͔͆̂̀ñ̮͈̗͚̱̠͡d̡͔́̂͊ͬ̋͜,̾ͯͤ͛̐̒͢͝ a̷̲̗̯͌͋̆̇n̨͎̮͖̊̌̄ͬd͍̹̺̼̟̀̽̿ I͍̻̙͗̒ͫ͢͜ q̺͈̜̹͔͑̚͢ų͔̜̠̙͖͓̍oͫ͏̶̲̖̥̅ͩt̝̭̜̠̟̎̄́e͖̥̟͍͆͐̀́ t̩̯̝̥͌ͦ̐̉h̶͚̝̖̜͗͊͐e̵̡̬͖̜͈̽̓ f͈͎̳̊̂ͩ̽͘į̥͖̤̙̑̄̓g̼͚͉̞ͦ̒̿̚h̡̨̹͕̀̏͡͠t̵̩̰̘͚͆̉͋s͏̶̘͖̯̐͗̓ ẖ̵͔ͨ̋̎͒͘i̪̬̘̮̻ͩ̇͠s̵͚̩͔̹͋ͨ͢t̲̯̭̹̼̖̄̏o̧͎̣̎̋ͨ̓ͩr̰̦̣ͥͮ̾ͤͅi̖̳͓̋́̉ͬ͘ç͎̬̀ͤ̔͟͝a̛̦̺̍ͧ̉͜͠l̴̨͍͈͖ͪ͏̢
```

It will attempt to corrupt emoji, but it doesn't work all that well:
```
❯ icorrupt "It does weird things with emoji though 😍"
I̸̳̱̯͗̏ͪ͞t̹ͦ͏̵̘̮̋ͫ ḓ̳̪̐̊ͫͬ͑õ͙͚̮̱̲͠ͅe̵̷̜͇ͦ̅̊͘s͈̱̩ͬ́̆ͭ͢ w̬̖͎ͦ̔͗ͮ͢e̡̛͓̮̒ͯ͘ͅi͓͚͕̬ͫ͌ͥ͡r̵̢͙̀ͥ͐̿ͫd̵̜͎̦̝̤͔̖ t̮͒̐͏͊̈̽̈́h͇͍̗̀͛̋͞͡ì̲͎̹̂͊̉͝n̷͇̘ͣ̑̄̅͡g̛̹͈̐̒̕͘͡s̸̨̤̞̻̀̀͢ w̢̳̗͎ͪ̓͋̇i̞̲̱ͭ̐ͨ͑̈t̷̙͖ͩͦͣ͒͜h͏̡͎̼ͩͮ͢͡ e̶̦͎̙̬̔͟͝m͇̗͉͖͐̈̚͝ő̵̖̭̘̊̈̏j̥̖̺̋ͦ̎̆͢i̧͈͎̣ͪ̂ͨ͠ t̨̧̺̥͇͍͈̲h̜̩̐͑ͩ̾͢ͅò͈̭̣̄̆͟͠ụ̶̷̟̹͊̈́ͪg̛̠͓͖͋ͤ̽̚hͥ́ͧ͐ͤ̐̇͝ 😍̷̡̰̱̀ͦ̃̏
```
