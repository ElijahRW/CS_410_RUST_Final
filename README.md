# CS 410 RUST Final
An XML parser to generate a game GUI


## Overview: _Design Stage_
In short, this repository is an attempt to develop a library capable of separating a game GUI from 
the actual game display. In many modern game engines this is a standard approach. 

This allows game developers to fine tune UI results without constant recompilation as well as 

XML files will contain button implementation details and settings. Each time a program is reloaded 
(or a command manually triggered), the XML file is reparsed and the buttons, healthbars, menus are 
reinstantiated (Allowing for dynamic configuration).

_Inspiration/Reference Using concepts from Cryengine 5: http://docs.cryengine.com/display/CEPROG/UI+Element_

## XML Syntax
Currently, XML files must adhere to a standardized syntax to be properly read into the parser currently. 
An refer to example_button.xml for a demonstration of this syntax.

#To view functionality
By default, by running `cargo run` this project will load the beginnings of a
simple pong game. Custom defined UI buttons will appear on the screen.

*With the program running, by pressing the `'r'` key the program will trigger a reload.
prompting the program to reparse the nested xml files to redraw the UI. Therefore, without
recompiling the code, custom buttons may be adjusted and reloaded back into the window.

#Key Features:
* Reload XML asset files by either reloading file (no compilation needed) or pressing the `'r'` key
* All buttons defined in `example_button_array.xml` are clickable. only id. New buttons can be defined
 and reloaded using description outlined above. Reference example XML file for example elements.
* Minor ui Implemented (Custom player paddle button may move up and down. Using the up and down arrow buttons)
* Animated ball that travels at an angle (Stretch goal was to implement ricochet)



# Development Plan: 
_Outline Will be copied into the project PDF_
### Weeks 4-5: Research:
* Definition of elements: (Such as)
  * XML syntax expected by the Parser.
  * Desired UI elements: (Such as buttons, health bars, etc)
     * The capabilities of these
     
* Research: 
  * Available Rust engines
  * Available Rust parsing tools _I'd honestly rather not write my own XML 
  parser with so many useful ones available
  
#### Week 6: Development of Parser
#### Week 7: Development of Translator
#### Week 8: Development of Example Game
* Time permitting, a simple game will be developed to demonstrate the functionality of the UI system
   * Likely Will be a simple box collecting game.
   
   
###Development Postmortem

To a certain extent. I'd claim that I overestimated the difficulty I would encounter in developing this project.
I encountered several issues integrating with the test engine. I admittedly struggled to find an effective
method of implementing my XML system into the Piston engine. Rendering objects in their system requires full adoption of their trait heirarchy,
 making an autonomous XML hierarchy difficult to maintain.

In future development I would likely attempt the following:
* Add coverage to prevent panics at parsing attempts: this would require matching at all 
unwrap calls. Simple but requires defaults prepared.
* Convert vector of asset path's into a Hash set for faster reference. 

#Other references:
* Code utilized to start with piston can be found under crate's example projects. 
Current code should have little to no parallels now.