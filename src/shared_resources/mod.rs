use cortex_m;
use cortex_m::interrupt::Mutex;
use stm32f4::stm32f407;
use core::cell::RefCell;
use lazy_static::lazy_static;
use stm32f407::{EXTI, SYSCFG};

lazy_static!
{
    pub static ref MUTEX_EXTI:  Mutex<RefCell<Option<stm32f407::EXTI>>>  = Mutex::new(RefCell::new(None));
    pub static ref MUTEX_SYSCFG: Mutex<RefCell<Option<stm32f407::SYSCFG>>> = Mutex::new(RefCell::new(None));
}

pub fn init(exti: EXTI, syscfg: SYSCFG)
{
    /* Put the resources in mutexes */
    cortex_m::interrupt::free(|cs|
    {
        MUTEX_EXTI.borrow(cs).replace(Some(exti));
        MUTEX_SYSCFG.borrow(cs).replace(Some(syscfg));
    });
}
