#include "lzfx/include/lzfx.h"
#include "lzfx/include/lzfxbridge.h"
#include "lzfx/src/lib.rs.h"

uint32_t compress(const rust::Vec<uint8_t> &data, rust::Vec<uint8_t> &out)
{
    std::vector<uint8_t> databuf(data.begin(), data.end());
    std::vector<uint8_t> outbuf;
    outbuf.resize(out.capacity());
    unsigned int olen = out.capacity();
    int ret = lzfx_compress(&databuf[0], databuf.size(), &outbuf[0], &olen);
    outbuf.resize(olen);
    std::copy(
        outbuf.begin(), outbuf.end(),
        std::back_inserter(out));
    return olen;
}

uint32_t decompress(const rust::Vec<uint8_t> &data, rust::Vec<uint8_t> &out)
{

    std::vector<uint8_t> databuf(data.begin(), data.end());
    std::vector<uint8_t> outbuf;
    outbuf.resize(out.capacity());
    unsigned int olen = out.capacity();
    int ret = lzfx_decompress(&databuf[0], databuf.size(), &outbuf[0], &olen);
    outbuf.resize(olen);
    std::copy(
        outbuf.begin(), outbuf.end(),
        std::back_inserter(out));
    return olen;
}