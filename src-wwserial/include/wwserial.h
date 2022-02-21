#pragma once
#include "rust/cxx.h"

#include <memory>
#include <string>

struct CxxSerial
{
    CxxSerial(rust::String port, uint32_t baud, uint32_t timeout, bool hardware);

    uint32_t write(const rust::Vec<uint8_t> &data) const;

    uint32_t read(rust::Vec<uint8_t> &data, uint32_t cap) const;

    void flush() const;

    bool check() const;

private:
    struct impl;
    std::shared_ptr<impl> impl;
};

// port: String, baud: u32, timeout: u32, hardware: bool
std::unique_ptr<CxxSerial> new_cxx_serial(rust::String port, uint32_t baud, uint32_t timeout, bool hardware);