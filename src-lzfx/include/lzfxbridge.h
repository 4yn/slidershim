#pragma once
#include "rust/cxx.h"

uint32_t compress(const rust::Vec<uint8_t> &data, rust::Vec<uint8_t> &out);

uint32_t decompress(const rust::Vec<uint8_t> &data, rust::Vec<uint8_t> &out);