use core::intrinsics::{volatile_load,volatile_store};

// Hm, I get bus error when I try to use these addresses on AHB.
// Probably some extra init is needed to use the AHB, to be researched.
//    pub const GPIO_PORTA_BASE: uint = 0x40058000;
//    pub const GPIO_PORTB_BASE: uint = 0x40059000;
//    pub const GPIO_PORTC_BASE: uint = 0x4005A000;
//    pub const GPIO_PORTD_BASE: uint = 0x4005B000;
//    pub const GPIO_PORTE_BASE: uint = 0x4005C000;
//    pub const GPIO_PORTF_BASE: uint = 0x4005D000;
//    pub const GPIO_PORTG_BASE: uint = 0x4005E000;
//    pub const GPIO_PORTH_BASE: uint = 0x4005F000;

// These are the addresses for the APB bus.
pub const PORTA_BASE: uint = 0x40004000;
pub const PORTB_BASE: uint = 0x40005000;
pub const PORTC_BASE: uint = 0x40006000;
pub const PORTD_BASE: uint = 0x40007000;
pub const PORTE_BASE: uint = 0x40024000;
pub const PORTF_BASE: uint = 0x40025000;
pub const PORTG_BASE: uint = 0x40026000;
pub const PORTH_BASE: uint = 0x40027000;
const GPIODATA:   uint = 0x000;
const GPIODIR:    uint = 0x400;
const GPIOIS:     uint = 0x404;
const GPIOIBE:    uint = 0x408;
const GPIOIEV:    uint = 0x40c;
const GPIOIM:     uint = 0x410;
const GPIORIS:    uint = 0x414;
const GPIOMIS:    uint = 0x418;
const GPIOICR:    uint = 0x41c;
const GPIOAFSEL:  uint = 0x420;
const GPIODR2R:   uint = 0x500;
const GPIODR4R:   uint = 0x504;
const GPIODR8R:   uint = 0x508;
const GPIOODR:    uint = 0x50c;
const GPIOPUR:    uint = 0x510;
const GPIOPDR:    uint = 0x514;
const GPIOSLR:    uint = 0x518;
const GPIODEN:    uint = 0x51c;
const GPIOLOCK:   uint = 0x520;
const GPIOCR:     uint = 0x524;
const GPIOAMSEL:  uint = 0x528;
const GPIOPCTL:   uint = 0x52c;
const GPIOADCCTL: uint = 0x530;
const GPIODMACTL: uint = 0x534;

#[inline]
fn set(port: uint, offset: uint, value: u32) {
    unsafe {
        volatile_store((port + offset) as *mut u32, value)
    }
}

#[inline]
fn get(port: uint, offset: uint) -> u32 {
    unsafe {
        volatile_load((port + offset) as *const u32)
    }
}

#[inline]
pub fn enable(port: uint) {
    // To enable a GPIO, need to set its bit in SYSCTL_RCGC2
    const SYSCTL_RCGC2: uint = 0x400FE108;
    let gpio_num =
        if port >= PORTE_BASE {
            4 + ((port - PORTE_BASE) >> 12)
        } else {
            (port - PORTA_BASE) >> 12
        };
    let v = unsafe { volatile_load(SYSCTL_RCGC2 as *const u32) };
    unsafe {
        volatile_store(SYSCTL_RCGC2 as *mut u32,
                       v | (1u32 << (gpio_num)))
    }
}

#[inline]
pub fn port_direction(port: uint, mask: u32) {
    set(port, GPIODEN, mask);
    set(port, GPIODIR, mask)
}

pub struct Pin {
    nr: u32,
}

impl Pin {
    fn basemask(self, port: uint) -> (uint, u32) {
        let n = self.nr as uint;
        (port, 1u32 << (2 + (n % 8)))
    }

    pub fn input(self, port: uint) {
        let n = self.nr as uint;
        let mask = 1u32 << (n % 8);
        unsafe {
            let v = volatile_load((port + GPIODIR) as *const u32);
            volatile_store((port + GPIODIR) as *mut u32, v & !mask);
        }
    }

    pub fn output(self, port: uint) {
        let n = self.nr as uint;
        let mask = 1u32 << (n % 8);
        unsafe {
            let v = volatile_load((port + GPIODIR) as *const u32);
            volatile_store((port + GPIODIR) as *mut u32, v | mask);
        }
    }

    pub fn high(self, port: uint) {
        let (base, mask) = self.basemask(port);
        unsafe {
            volatile_store((base + mask as uint) as *mut u32, 0xFF)
        }
    }

    pub fn low(&self, port: uint) {
        let (base, mask) = self.basemask(port);
        unsafe {
            volatile_store((base + mask as uint) as *mut u32, 0x0)
        }
    }
}

pub const PIN0:  Pin = Pin{ nr: 0 };
pub const PIN1:  Pin = Pin{ nr: 1 };
pub const PIN2:  Pin = Pin{ nr: 2 };
pub const PIN3:  Pin = Pin{ nr: 3 };
pub const PIN4:  Pin = Pin{ nr: 4 };
pub const PIN5:  Pin = Pin{ nr: 5 };
pub const PIN6:  Pin = Pin{ nr: 6 };
pub const PIN7:  Pin = Pin{ nr: 7 };
