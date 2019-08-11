use stm32f4xx_hal::gpio::gpioa::{PA0, Parts};
use stm32f4xx_hal::gpio::{ExtiPin, Edge, PullDown, Input};
use lazy_static::lazy_static;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_semihosting::hprintln;
use stm32f4::stm32f407::interrupt;
use crate::shared_resources::{MUTEX_EXTI, MUTEX_SYSCFG};

lazy_static!
{
    static ref MUTEX_BUTTON: Mutex<RefCell<Option<PA0<Input<PullDown>>>>>  = Mutex::new(RefCell::new(None));    
}

#[interrupt]
fn EXTI0()
{
    hprintln!("Interrupt!!").unwrap();

    cortex_m::interrupt::free(|cs|
    {
        let mut button = MUTEX_BUTTON.borrow(cs).borrow_mut();
        let mut exti = MUTEX_EXTI.borrow(cs).borrow_mut();

        button.as_mut().unwrap().clear_interrupt_pending_bit(exti.as_mut().unwrap());
    });
}


pub fn init(gpioa: Parts)
{
    let mut button = gpioa.pa0.into_pull_down_input();

    cortex_m::interrupt::free(|cs|
    {
        let mut exti_ref = MUTEX_EXTI.borrow(cs).borrow_mut();
        let mut syscfg_ref = MUTEX_SYSCFG.borrow(cs).borrow_mut();
        let exti = exti_ref.as_mut().unwrap();
        let syscfg = syscfg_ref.as_mut().unwrap();

        button.make_interrupt_source(syscfg);
        button.trigger_on_edge(exti, Edge::FALLING);
        button.enable_interrupt(exti);
        MUTEX_BUTTON.borrow(cs).replace(Some(button));
    });
}
