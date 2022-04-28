# CPU

A very very simple Virtual CPU. Not Turing complete.

## Codes

```txt
0x00 Setr // Set active registry
0x01 Load // Load value from address into active registry
0x02 Store // Store value from active registry into memory at address
0x03 Add // Add value from address to the active registry
0x04 Sub // Subtract value out of address from the active registry
0x05 Mul // Multiply the value in the active registry by the value from address
0x06 Div // Divide de value in the active registry by the value from address
0x0A Jump // Jump to the address that is stored at address
```

## Instructions

```
0x0001
  ^^------------- Oppcode

0x0001
    ^^----------- Operand
```

This piece of code would set the active registry to be registry 1.

## Example

```txt
// Simple program that counts until there's an overflow.

// SETUP:0
0x000A // Setr select active registry 00
0x0105 // Load the value from 0x05 into the active reg

// LOOP:2
0x0307 // Add value in 07 to active reg
0x0205 // Store value from active reg in 05
0x0A08 // Jump to value store in 08

// VARIABLES:5
0x0000
0x0000
0x0001
0x0001
```
