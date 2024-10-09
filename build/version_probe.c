#include <blkid/blkid.h>

#define VERSION2(V) "RUST_BLKID_VERSION_"#V
#define VERSION(V) VERSION2(V)

#ifdef BLKID_VERSION
VERSION(BLKID_VERSION)
#endif
