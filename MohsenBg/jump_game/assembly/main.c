#include <stdio.h>
#include <stdbool.h>

// Declare the assembly function
extern int jump_game(int *nums, int numsSize);

// Function to print the array
void print_array(int *arr, int size) {
    printf("[ ");
    for (int i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("]\n");
}

int main() {
    int nums1[] = {2, 3, 1, 1, 4};
    int nums2[] = {3, 2, 1, 0, 4};

    int size1 = sizeof(nums1) / sizeof(nums1[0]);
    int size2 = sizeof(nums2) / sizeof(nums2[0]);

    // Print and test nums1
    printf("===========================\n");
    printf("Test 1: Array ");
    print_array(nums1, size1);
    printf("%s\n", jump_game(nums1, size1) ? "Can jump" : "Cannot jump");
    printf("===========================\n");

    // Print and test nums2
    printf("\n===========================\n");
    printf("Test 2: Array ");
    print_array(nums2, size2);
    printf("%s\n", jump_game(nums2, size2) ? "Can jump" : "Cannot jump");
    printf("===========================\n");
    
    return 0;
}