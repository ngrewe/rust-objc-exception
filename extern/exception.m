#include <objc/objc.h>
#if __has_include("objc/NSObject.h")
#include <objc/NSObject.h>
#else
#include <Foundation/NSObject.h>
#endif
void RustObjCExceptionThrow(id exception) {
    @throw exception;
}

int RustObjCExceptionTryCatch(void (*try)(void *), void *context, id *error) {
    @try {
        try(context);
        if (error) {
            *error = nil;
        }
        return 0;
    } @catch (id exception) {
        if (error) {
            *error = [exception retain];
        }
        return 1;
    }
}
