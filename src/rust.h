#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// Computes the dot product of two arrays.
///
/// # Safety
///
/// The input arrays (`arr1` and `arr2`) must be valid non-null pointers pointing to arrays of
/// at least `len` elements. Accessing invalid memory through these pointers can lead to
/// undefined behavior.
///
/// The function assumes that the memory pointed to by `arr1` and `arr2` is properly allocated
/// and that the length `len` is accurate.
int32_t dot_product(const int32_t *arr1, const int32_t *arr2, uintptr_t len);

/// Computes the elementwise product of two arrays.
///
/// # Safety
///
/// The input arrays (`arr1` and `arr2`) must be valid non-null pointers pointing to arrays of
/// at least `len` elements. Accessing invalid memory through these pointers can lead to
/// undefined behavior.
///
/// The function allocates memory for the result using `Vec`, and the caller is responsible
/// for releasing this memory using appropriate means (e.g., `free` in C).
int32_t *elementwise_product(const int32_t *arr1, const int32_t *arr2, uintptr_t len);

} // extern "C"
