#include <sys/types.h>
#include <errno.h>

// These addresses are hard coded. A much better strategy would be have them
// exported from the linker script, and have the linker do some size checking
// against the data, bss and whatever else are in RAM.
const static char * heap_start    = (char *) 0x20002000;
const static char * heap_end      = (char *) 0x20005000;
char * heap_top = (char *) 0x20002000;
//char * dummy;
caddr_t _sbrk(int incr)
{
    // Need to make this static.
    //static char * heap_top = (char *) 0x20002000;
    char * prev_heap_top = heap_top;
    
    //dummy = 0;
    
    if (heap_top + incr >= heap_end) {
        // The next increment will blow the heap. Don't allow it.
        errno = ENOMEM;
        return (caddr_t) -1;
    }
    
    // Increase the size of the heap.
    heap_top += incr;
    
    //Return the previous top of the heap.    
    return (caddr_t) prev_heap_top;
}