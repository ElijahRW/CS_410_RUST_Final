# CS 410 RUST Final
An XML parser to generate a game GUI:


## Overview:
In short, this repository is an attempt to develop a library capable of separating a game GUI from 
the actual game display. In many modern game engines this is a standard approach. 

XML files will contain button implementation details and settings. Each time a program is reloaded 
(or a command manually triggered), the XML file is reparsed and the buttons, healthbars, menus are 
reinstantiated (Allowing for dynamic configuration).

Inspiration/Reference Using concepts from Cryengine 5: http://docs.cryengine.com/display/CEPROG/UI+Element


##XML Syntax
Currently, XML files must adhere to a standardized syntax to be properly read into the parser currently. 
However
