#include "wwserial/include/serial.h"
#include "wwserial/include/wwserial.h"
#include "wwserial/src/lib.rs.h"

#include <algorithm>

struct CxxSerial::impl
{
    friend CxxSerial;
    bool ok;
    std::shared_ptr<serial::Serial> serial_port;

    impl();
};

CxxSerial::impl::impl()
    : serial_port(nullptr){};

CxxSerial::CxxSerial(rust::String port, uint32_t baud, uint32_t timeout, bool hardware)
    : impl(new struct CxxSerial::impl)
{
    impl->ok = true;
    std::string port_stdstring(port.c_str());
    try
    {
        impl->serial_port = std::shared_ptr<serial::Serial>(
            new class serial::Serial(port_stdstring, baud, serial::Timeout::simpleTimeout(timeout)));
        if (hardware)
        {
            impl->serial_port->setFlowcontrol(serial::flowcontrol_hardware);
        }
    }
    catch (...)
    {
        impl->ok = false;
    }
}

uint32_t CxxSerial::write(const rust::Vec<uint8_t> &data) const
{
    if (impl->ok && impl->serial_port->isOpen())
    {
        std::vector<uint8_t> buf(data.begin(), data.end());
        return impl->serial_port->write(buf);
    }
    return 0;
};

uint32_t CxxSerial::read(rust::Vec<uint8_t> &data, uint32_t cap) const
{
    if (impl->ok && impl->serial_port->isOpen())
    {
        std::vector<uint8_t> buf;
        buf.reserve(cap);
        size_t bytes_read = impl->serial_port->read(buf, (size_t)cap);
        std::copy(
            buf.begin(), buf.end(),
            std::back_inserter(data));
        return bytes_read;
    }
    return 0;
};

void CxxSerial::flush() const
{
    if (impl->ok && impl->serial_port->isOpen())
    {
        return impl->serial_port->flush();
    }
};

bool CxxSerial::check() const
{
    return impl->ok;
}

std::unique_ptr<CxxSerial> new_cxx_serial(rust::String port, uint32_t baud, uint32_t timeout, bool hardware)
{
    return std::make_unique<CxxSerial>(port, baud, timeout, hardware);
}
