# CS 410 RUST Final
An XML parser to generate a game GUI:


## Overview:
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


## Development Plan: 
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
  

### Week 6: Development of Parser

### Week 7: Development of Translator

### Week 8: Development of Example Game
* Time permitting, a simple game will be developed to demonstrate the functionality of the UI system
   * Likely Will be a simple box collecting game.
* Finishing off any incomplete elements