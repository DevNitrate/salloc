# Salloc

Salloc is a safe to call rust library to allocate memory using an allocator which uses the HeapApi from win32 api and uses C function names for easier use.

**NOTE:** This library only works on windows for now.

If you happen to actually unironically use this library and you find an issue you can open an issue or contact me on discord: ``devnitrate`` (i might not see the issue so in that case also contact me on discord to notify me)

# Documentation

## The Sallocator struct

To allocate memory, you need an allocator that can call the memory allocating functions and it is the base of this library. The allocator acts a bit like the pointer to the allocated memory (even if it is not the case).

## The HeapError enum

The **HeapError** enum is used to return errors from functions used by **Sallocator** in which every function returns a Result where the Error type is **HeapError**

## Sallocator::new() -> Result<Self, HeapError>

The new function is used to initialize a new allocator with default values. This function returns ``Sallocator`` but can return ``HeapError::SallocCreateFailed`` if the HeapCreate() function from win32 fails:

```rust
let allocator: Sallocator<u32> = Sallocator::new().unwrap();
```

## Sallocator::malloc(&mut self, size: usize)

The malloc function takes ``size`` which is the size of memory to allocate in bytes and works like calloc in C. This function returns ``()`` but can return ``HeapError::SallocMallocFailed`` if the HeapAlloc() function from win32 fails:

```rust
allocator.malloc(size_of::<u32>()).unwrap();
```

>This snippet allocates the memory for one u32 value (4 bytes)

## Sallocator::realloc(&mut self, size: usize) -> Result<(), HeapError>

The realloc function takes ``size`` which is the new size of memory to allocate in bytes and works like realloc in C. This function returns ``()`` but can return ``HeapError::SallocReallocFailed`` if the HeapReAlloc() function from win32 fails:

```rust
allocator.realloc(size_of::<u32>() * 2).unwrap();
```

>This snippet reallocates the previous 4 bytes into 8 bytes (size of 2 u32 values)

## Sallocator::set(&mut self, index: isize, el: T) -> Result<(), HeapError>

The set function takes in ``index`` which is the index at which the element ``el`` will be set inside the block of memory. This function returns ``()`` but can return either ``HeapError::SallocSetUnallocated`` if you try to use this function when you didn't allocate any memory or ``HeapError::SallocSetOutOfBound`` if you try to set a value in a region of memory outside of what you allocated or ``index`` is smaller than 0:

```rust
allocator.set(0, 123).unwrap();
```

>This snippet sets the index 0 to a value of 123

## Sallocator::get(&mut self, index: isize) -> Result<T, HeapError>

The set function takes in ``index`` which is the index at which the value will be accessed in the region of allocated memory. This function returns ``T`` but can return either ``HeapError::SallocGetUnallocated`` if you try to use this function when you didn't allocate any memory or ``HeapError::SallocGetOutOfBound`` if you try to access a value in a region of memory outside of what you allocated or ``index`` is smaller than 0:

```rust
let val: u32 = allocator.get(0).unwrap();
```

>This snippet gets the value at index 0 which was set in previous example so ``val == 123``

## Sallocator::free(&mut self) -> Result<(), HeapError>

The free function frees all the memory allocated previously but doesn't invalidate the allocator. This function returns ``()`` but can return either ``HeapError::SallocFreeUnallocated`` if you try to free memory when you haven't allocated any or ``HeapError::SallocFreeFailed`` if the HeapFree() function from win32 fails:

```rust
allocator.free().unwrap();
```

>The snippet frees the 8 bytes we allocated in previous examples but the allocator can still be used later

## Sallocator::destroy(&mut self) -> Result<(), HeapError>

The destroy function invalidates the allocator which can no longer be used after (note: the free function doesn't necessarily need to be called before). This function returns ``()`` but can return ``HeapError::SallocDestroyFailed`` if the HeapDestroy() function from win32 fails:

```rust
allocator.destroy().unwrap();
```

>This snippet frees the 8 bytes previously allocated if the free function wasn't called before and then invalidates the allocator