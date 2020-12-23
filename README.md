# roll
A simple terminal-based dice roller, [re-written](https://github.com/jvlsg/roll) in Rust, for RPG and CLI enthusiasts
by João Victor Guimarães

## (UN)INSTALLING
### Using Cargo
Run 
`cargo install roll`

### Releases
See the releases tab in Github

## USAGE
`roll [options] XdY[+i-d...][#target] ...`
Where:

`X` is number of dice

`Y` is the type of die

`i` and `d` are optional increments and decrements, respectively. You can **use several** increments and decrements

`target` is a Target Number. A roll will be successful if Result >= TN
It is possible to make several rolls and use several modifiers to each roll.

Please note that roll **fails quietly** - Incorrect input will simply be ignored, unless you are using verbose mode. 

## OPTIONS
`-p`		Dice Pool mode: Each die of a roll is independent of the others

`-v`		Verbose mode

`-t<NUM>`	Default Target Number: All rolls will be compared against it.
		An explicit target number declaration will be used instead for that roll.
		
`-V`		Prints out the program's version\n"


## Examples and Features
### `roll XdY`
rolls X dYs and sums their results
    
    $: roll 2d10 3d4 1d123
    [13] [6] [88]
    
### `roll -p xdy`     
rolls X dYs as a dice pool, gives individual results for each die
    
    $: roll -p 2d10 3d4 1d123
    [10,7]
    [4,2,2]
    [58]
    
### `roll xdy+i-d`    
rolls X dYs sums their results, and modifies final sum with modifiers    
    
    $: roll 2d20+12-3+1-1-2+4#10
    [8 ➔ 19 ✓ ]

For dicepools: rolls X+i-d dYs.

    $: roll -p 3d6+2-3
    [2,2]
    
    $: roll -p 1d6-12
    []
    
### `roll xdy#target`   
rolls X dYs, returning success (✓) if results are Greater or Equal than a Target Number, else it returns a failure (✕)
    
    $: roll 2d10#15 3d4#3 1d123#1409
    [13 ✕ ]
    [4 ✓ ]
    [68 ✕ ]

For dicepools, each die that is greater or equal than the Target Number is considered a success.
    $: roll -p 3d6+2-3#3
    [4,1| 1 Success]
    
    $: roll -p 3d6+2-3#2000
    [1,5| 0 Successes]

### `roll xdy -t target`   
Sets a default target number 
    
    $: roll -t 5 -p 3d6 2d6#1
    [3,2,2| 0 Successes]
    [6,3| 2 Successes]

### `roll xdy -v`
A verbose mode. Also gives information on errors
```   
roll 3d6#4 -v -p
Dice Roll {
    Dice quantity: 3,
    Dice type: 6,
    Increment: 0,
    Decrement: 0,
    Target number: 4,
    Type: Pool,
    Results: [
        4,
        6,
        3,
    ],
    Final Result: 2,
    Bottomline: "[4,6,3| 2 Successes]",
}
```
```
roll 2d20+12-d3+1 -v
[!] 2d20+12-d3+1 Failed
Regex Error - your roll must follow the following pattern XdY[+A-B][#Z]
Run roll --help to learn more
```

Have fun!

## Authors
* **João Victor Guimarães** - [jvlsg](https://github.com/jvlsg/)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
