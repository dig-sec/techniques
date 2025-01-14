### Linux Version:
```python
import mmap
import ctypes

buffer_ = mmap.mmap(-1, mmap.PAGESIZE, prot=mmap.PROT_READ | mmap.PROT_WRITE | mmap.PROT_EXEC)
ftype = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.c_int)
fpointer = ctypes.c_void_p.from_buffer(buffer_)
function = ftype(ctypes.addressof(fpointer))
buffer_.write(
    b"\x8b\xc7"  # mov eax, edi
    b"\x83\xc0\x04"  # add eax, 4
    b"\xc3"  # ret
)

retval = function(58)
print(retval)  # => 62
del fpointer
buffer_.close()
```

### Windows Version:
```python
import mmap
import ctypes
import os

# Allocate a memory buffer with read, write, and execute permissions
buffer_ = mmap.mmap(-1, mmap.PAGESIZE, access=mmap.ACCESS_WRITE)

# Define the function type
ftype = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.c_int)

# Write the machine code to the buffer
buffer_.write(
    b"\x8b\xc7"  # mov eax, edi
    b"\x83\xc0\x04"  # add eax, 4
    b"\xc3"  # ret
)

# Get the address of the buffer
address = ctypes.addressof(ctypes.c_void_p.from_buffer(buffer_))

# Change the protection of the buffer to read, write, and execute
ctypes.windll.kernel32.VirtualProtect(ctypes.c_void_p(address), ctypes.c_size_t(mmap.PAGESIZE), 0x40, ctypes.byref(ctypes.c_ulong()))

# Create a callable function from the buffer
function = ftype(address)

# Call the function
retval = function(58)
print(retval)  # => 62

# Clean up
buffer_.close()
```

This Windows version includes adjustments to the `mmap` and memory protection settings appropriate for the Windows operating system. If you have any other questions or need further assistance, feel free to ask!