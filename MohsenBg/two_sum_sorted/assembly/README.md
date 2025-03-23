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

Make sure you're in the directory where `two_sum_sorted.asm` is located. Then, run the following command to assemble the assembly file into an object file:  

```bash
nasm -f elf64 two_sum_sorted.asm -o two_sum_sorted.o
```  

After running this command, you should have a file named `two_sum_sorted.o` in the same directory.  

### Step 4: Compile the C File with the Assembly Object File  

Now, compile the C file (`main.c`) along with the assembly object file:  

```bash
gcc -no-pie main.c two_sum_sorted.o -o two_sum_sorted
```  

### Step 5: Run the Program  

After successful compilation, an executable named `two_sum_sorted` will be created in the same directory. You can now run the program:  

```bash
./two_sum_sorted
```  

### Expected Output  

If everything is set up correctly, you should see output like this:  

```bash
===================================
Array: 1 2 3 4 5 
Test 1 Result: 1, 5
===================================
Array: 1 2 3 4 5 
Test 2: No solution found.
===================================
Array: 1 5 
Test 3 Result: 1, 2
===================================
Array: 
Test 4: No solution found.

```
