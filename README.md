# roll
A simple Diceroller, re-written in Rust, for RPG and CLI enthusiasts
by João Victor Guimarães

## (UN)INSTALLING
> Soon

## USAGE
`roll [options] XdY[+A-B#Z] ...`
Where:

`X` is number of dice

`Y` is the type of die

`A` and `B` are optional increments and decrements, respectively

`Z` is a Target Number. A roll will be successful if Result >= TN
It is possible to make several rolls and use several modifiers to each roll.

Please note that roll fails quietly - Incorrect input will simply be ignored. 

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
    
### `roll xdy+a-b`    
rolls X dYs sums their results, and modifies final sum with modifiers    
    
    $: roll 1d20+4-6 3d6-20+3
    [4 ➔ 2]
    [13 ➔ -4]

For dicepools: rolls X+a-b dYs.

    $: roll -p 3d6+2-3
    [2,2]
    
    $: roll -p 1d6-12
    []
    
### `roll xdy#z`   
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

### `roll xdy -t z`   
Sets a default target number of Z. 
    
    $: roll -t 5 -p 3d6 2d6#1
    [3,2,2| 0 Successes]
    [6,3| 2 Successes]
    
Have fun!

## Authors
* **João Victor Guimarães** - [jvlsg](https://github.com/jvlsg/)


## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
