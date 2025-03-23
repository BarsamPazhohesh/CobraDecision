#include <stdio.h>
#include <stdlib.h>

extern int* two_sum_sorted(int* numbers, int numbers_size, int target, int* returnSize);

void print_array(int* numbers, int numbers_size) {
    printf("Array: ");
    for (int i = 0; i < numbers_size; i++) {
        printf("%d ", numbers[i]);
    }
    printf("\n");
}

int main() {
    // Test case 1: Target sum is found
    {
        int numbers[] = {1, 2, 3, 4, 5};
        int numbers_size = 5;
        int target = 6;
        int returnSize = 0;

        printf("===================================\n");
        print_array(numbers, numbers_size);

        int* result = two_sum_sorted(numbers, numbers_size, target, &returnSize);

        if (result != NULL && returnSize == 2) {
            printf("Test 1 Result: %d, %d\n", result[0], result[1]);
            free(result);
        } else {
            printf("Test 1: No solution found.\n");
        }
    }

    // Test case 2: Target sum is not found
    {
        int numbers[] = {1, 2, 3, 4, 5};
        int numbers_size = 5;
        int target = 10;
        int returnSize = 0;

        printf("===================================\n");
        print_array(numbers, numbers_size);

        int* result = two_sum_sorted(numbers, numbers_size, target, &returnSize);

        if (result != NULL && returnSize == 2) {
            printf("Test 2 Result: %d, %d\n", result[0], result[1]);
            free(result);
        } else {
            printf("Test 2: No solution found.\n");
        }
    }

    // Test case 3: Single pair that sums to the target
    {
        int numbers[] = {1, 5};
        int numbers_size = 2;
        int target = 6;
        int returnSize = 0;

        printf("===================================\n");
        print_array(numbers, numbers_size);

        int* result = two_sum_sorted(numbers, numbers_size, target, &returnSize);

        if (result != NULL && returnSize == 2) {
            printf("Test 3 Result: %d, %d\n", result[0], result[1]);
            free(result);
        } else {
            printf("Test 3: No solution found.\n");
        }
    }

    // Test case 4: Empty array (edge case)
    {
        int numbers[] = {};
        int numbers_size = 0;
        int target = 6;
        int returnSize = 0;

        printf("===================================\n");
        print_array(numbers, numbers_size);

        int* result = two_sum_sorted(numbers, numbers_size, target, &returnSize);

        if (result != NULL && returnSize == 2) {
            printf("Test 4 Result: %d, %d\n", result[0], result[1]);
            free(result);
        } else {
            printf("Test 4: No solution found.\n");
        }
    }

    return 0;
}
