# Run Program  

**Note: This program is designed to run on `Linux` only. It may not work on Windows or macOS without modifications.**
### Step 1: Ensure NASM is Installed  

First, make sure NASM is installed. You can check by running the following command:  

```bash
nasm -v
```  

If NASM is installed, you should see output similar to this:  

```bash
NASM version 2.16.03 compiled on Jul 18 2024
```  

### Step 2: Ensure GCC is Installed  

Next, verify that GCC is installed by running:  

```bash
gcc -v
```  

The output should be something like this:  

```bash
gcc version 14.2.1 20250110 (Red Hat 14.2.1-7) (GCC)
```  

### Step 3: Navigate to the Directory  

Make sure you're in the directory where `jump_game.asm` is located. Then, run the following command to assemble the assembly file into an object file:  

```bash
nasm -f elf64 jump_game.asm -o jump_game.o
```  

After running this command, you should have a file named `jump_game.o` in the same directory.  

### Step 4: Compile the C File with the Assembly Object File  

Now, compile the C file (`main.c`) along with the assembly object file:  

```bash
gcc -no-pie main.c jump_game.o -o jump_game
```  

### Step 5: Run the Program  

After successful compilation, an executable named `jump_game` will be created in the same directory. You can now run the program:  

```bash
./jump_game
```  

### Expected Output  

If everything is set up correctly, you should see output like this:  

```
===========================
Test 1: Array [ 2 3 1 1 4 ]
Can jump
===========================

===========================
Test 2: Array [ 3 2 1 0 4 ]
Cannot jump
===========================
```

Now you have successfully compiled and run your program! 🎉  