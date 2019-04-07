use stm32f4xx_hal::gpio::gpiod::PD12;
use stm32f4xx_hal::gpio::gpiod::PD13;
use stm32f4xx_hal::gpio::gpiod::PD14;
use stm32f4xx_hal::gpio::gpiod::PD15;
use stm32f4xx_hal::gpio::PushPull;
use stm32f4xx_hal::gpio::Output;
use stm32f4xx_hal::gpio::gpiod::Parts;

pub struct LedColors
{
    pub green: PD12<Output<PushPull>>,
    pub orange: PD13<Output<PushPull>>,
    pub red: PD14<Output<PushPull>>,
    pub blue: PD15<Output<PushPull>>,
}

pub fn init(gpiod: Parts) -> LedColors
{
    let leds = LedColors
    {
        green: gpiod.pd12.into_push_pull_output(),
        orange: gpiod.pd13.into_push_pull_output(),
        red: gpiod.pd14.into_push_pull_output(),
        blue: gpiod.pd15.into_push_pull_output(),
    };

    return leds
}

// pub fn init(stm32f407_peripherals: &stm32::Peripherals)
// {
//     let rcc = stm32f407_peripherals.RCC.constrain();
//     let gpiod = stm32f407_peripherals.GPIOD.split();

//     let mut leds = LEDs
//     {
//         green: gpiod.pd12.into_push_pull_output(),
//         orange: gpiod.pd13.into_push_pull_output(),
//         red: gpiod.pd14.into_push_pull_output(),
//         blue: gpiod.pd15.into_push_pull_output(),
//     };

//     return leds
// }
