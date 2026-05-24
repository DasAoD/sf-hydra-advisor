// Nur Crunch-Dekomprimierung – DXT5-Dekodierung übernimmt Rust/image-Crate
#include "unitycrunch.h"
#include <cstdint>

extern "C" {

/// Crunch → rohe DXT5-Bytes. *out_data mit crunch_free() freigeben.
bool crunch_unpack(const uint8_t* data, uint32_t size,
                   uint8_t** out_data, uint32_t* out_size) {
    void* raw = nullptr;
    uint32_t raw_size = 0;
    bool ok = unity_crunch_unpack_level(data, size, 0, &raw, &raw_size);
    if (ok) {
        *out_data = reinterpret_cast<uint8_t*>(raw);
        *out_size = raw_size;
    }
    return ok;
}

void crunch_free(uint8_t* ptr) {
    delete[] ptr;
}

} // extern "C"
