#include <iostream>

extern "C" {
    int dot_product(const int* arr1, const int* arr2, size_t len);
    int* elementwise_product(const int* arr1, const int* arr2, size_t len);
}

int main() {
    int arr1[] = {1, 2, 3};
    int arr2[] = {4, 5, 6};
    size_t len = sizeof(arr1) / sizeof(arr1[0]);

    int dotProduct = dot_product(arr1, arr2, len);
    std::cout << "Dot Product: " << dotProduct << std::endl;

    int* elementwiseProduct = elementwise_product(arr1, arr2, len);
    std::cout << "Elementwise Product: ";
    for (size_t i = 0; i < len; i++) {
        std::cout << elementwiseProduct[i] << " ";
    }
    std::cout << std::endl;

    return 0;
}
