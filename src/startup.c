#include <string.h>
#include <stdint.h>

extern char _bss;     // The start of the bss region.
extern char _ebss;    // The end of he bss region.
extern char _ldata;
extern char _data;
extern char _edata;

/*
    Initialize the entire bss section to 0.
*/
void zero_fill_bss() {
    memset(&_bss, 0, &_ebss - &_bss);
}

/*
    Initialize the data section to values stored in flash at _ldata.
*/
void copy_initialized_data() {
    memcpy(&_data, &_ldata, &_edata - &_data);
}